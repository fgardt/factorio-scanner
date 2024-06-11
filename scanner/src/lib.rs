use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env, fs,
    hash::{Hash, Hasher},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use error_stack::{ensure, report, Context, Result, ResultExt};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use image::{codecs::png, imageops, ImageEncoder};
use imageproc::geometric_transformations::{self, rotate_about_center};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::{debug, error, field, info, info_span, instrument, warn};

use blueprint::{ConnectionDataExt, SignalID};
use mod_util::{
    mod_info::{DependencyVersion, Version},
    mod_list::ModList,
    mod_loader::Mod,
    mod_settings::SettingsDat,
    AnyBasic, DependencyList, UsedMods, UsedVersions,
};
use prototypes::{
    entity::{InserterPrototype, Type as EntityType, WallPrototype},
    tile::TilePrototype,
    ConnectedEntities, DataRaw, DataUtil, DataUtilAccess, EntityWireConnections,
    InternalRenderLayer, RenderLayerBuffer, TargetSize,
};
use types::{
    ConnectedDirections, Direction, ImageCache, MapPosition, RenderableGraphics,
    SimpleGraphicsRenderOpts, Vector,
};

pub mod bp_helper;
pub mod preset;

#[derive(Debug)]
pub enum ScannerError {
    SetupError,
    RenderError,
    NoBlueprint,
    ServerError,
}

impl Context for ScannerError {}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetupError => write!(f, "setup error"),
            Self::RenderError => write!(f, "render error"),
            Self::NoBlueprint => write!(f, "no blueprint"),
            Self::ServerError => write!(f, "server error"),
        }
    }
}

#[allow(clippy::too_many_lines)]
#[instrument(skip_all)]
pub fn get_protodump(
    factorio_userdir: &Path,
    factorio_bin: &Path,
    mod_list: &ModList,
    (bp_settings, bp_version): (&BTreeMap<String, AnyBasic>, u64),
) -> Result<DataRaw, ScannerError> {
    // check if cached dump exists and load it if available
    let cached_path = {
        let (active_mods, load_order) = mod_list.active_with_order();
        let mut hash = rustc_hash::FxHasher::default();
        for mod_name in &load_order {
            let Some(m) = active_mods.get(mod_name) else {
                continue;
            };
            format!("{}@{}", m.info.name, m.info.version).hash(&mut hash);
        }
        let mods_hash = hash.finish();

        let mut active_settings = bp_settings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>();
        active_settings.sort();

        let mut hash = rustc_hash::FxHasher::default();
        for setting in &active_settings {
            setting.hash(&mut hash);
        }
        let settings_hash = hash.finish();

        let cached_path = factorio_userdir.join(format!(
            "script-output/cached-dump_{mods_hash:X}-{settings_hash:X}.json.deflate"
        ));

        if cached_path.exists() {
            info!("loading cached prototype dump");
            let mut deflate = ZlibDecoder::new(
                fs::File::open(&cached_path)
                    .change_context(ScannerError::SetupError)
                    .attach_printable(format!(
                        "failed to open cached prototype dump at {cached_path:?}"
                    ))?,
            );

            let mut uncompressed = Vec::new();

            deflate
                .read_to_end(&mut uncompressed)
                .change_context(ScannerError::SetupError)
                .attach_printable(format!(
                    "failed to decompress cached prototype dump at {cached_path:?}"
                ))?;

            return DataRaw::load_from_bytes(&uncompressed)
                .change_context(ScannerError::SetupError);
        }

        cached_path
    };

    mod_list.save().change_context(ScannerError::SetupError)?;
    debug!("updated mod-list.json");

    SettingsDat::load_bp_settings(
        bp_settings,
        bp_version,
        factorio_userdir.join("mods/mod-settings.dat"),
    )
    .change_context(ScannerError::SetupError)?
    .save()
    .change_context(ScannerError::SetupError)?;
    debug!("updated mod-settings.dat");

    debug!("executing {factorio_bin:?} with --dump-data");
    let dump_out = Command::new(factorio_bin)
        .arg("--dump-data")
        .output()
        .change_context(ScannerError::SetupError)?;

    if dump_out.status.success() {
        debug!("prototype dump success");
    } else {
        return Err(report!(ScannerError::SetupError)
            .attach_printable(format!(
                "prototype dump failed with exit code {}",
                dump_out.status.code().unwrap_or(-1)
            ))
            .attach_printable(String::from_utf8_lossy(&dump_out.stdout).to_string()));
    }

    let dump_path = factorio_userdir.join("script-output/data-raw-dump.json");
    let dump_bytes = fs::read(&dump_path)
        .change_context(ScannerError::SetupError)
        .attach_printable(format!("failed to read prototype dump at {dump_path:?}"))?;

    // store minified + deflated version of dump in script-output folder
    {
        let minified = serde_json::to_vec(
            &serde_json::from_slice::<serde_json::Value>(&dump_bytes)
                .change_context(ScannerError::SetupError)
                .attach_printable("failed to minify prototype dump")?,
        )
        .change_context(ScannerError::SetupError)
        .attach_printable("failed to minify prototype dump")?;

        let mut deflate = ZlibEncoder::new(
            fs::File::create(&cached_path)
                .change_context(ScannerError::SetupError)
                .attach_printable(format!(
                    "failed to create cached prototype dump at {cached_path:?}"
                ))?,
            flate2::Compression::best(),
        );

        deflate
            .write_all(&minified)
            .change_context(ScannerError::SetupError)
            .attach_printable(format!(
                "failed to compress cached prototype dump at {cached_path:?}"
            ))?;
    }

    DataRaw::load_from_bytes(&dump_bytes).change_context(ScannerError::SetupError)
}

#[must_use]
#[instrument(skip_all, fields(entities = bp.entities.len(), tiles = bp.tiles.len()))]
pub fn calculate_target_size(
    bp: &blueprint::Blueprint,
    data: &DataUtil,
    target_res: f64,
    min_scale: f64,
) -> Option<TargetSize> {
    const TILE_RES: f64 = 32.0;

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for entity in &bp.entities {
        let Some(e_proto) = data.get_entity(&entity.name) else {
            continue;
        };

        let e_pos: MapPosition = (&entity.position).into();
        let c_box = e_proto.drawing_box();

        let tl = e_pos + c_box.top_left();
        let br = e_pos + c_box.bottom_right();

        if tl.x() < min_x {
            min_x = tl.x();
        }

        if tl.y() < min_y {
            min_y = tl.y();
        }

        if br.x() > max_x {
            max_x = br.x();
        }

        if br.y() > max_y {
            max_y = br.y();
        }
    }

    for tile in &bp.tiles {
        if data.get_proto::<TilePrototype>(&tile.name).is_none() {
            continue;
        }

        let t_pos: MapPosition = (&tile.position).into();
        let (x, y) = t_pos.as_tuple();

        if x < min_x {
            min_x = x;
        }

        if y < min_y {
            min_y = y;
        }

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    let min_x = (min_x - 0.5).floor();
    let min_y = (min_y - 0.5).floor();
    let max_x = (max_x + 0.5).ceil();
    let max_y = (max_y + 0.5).ceil();

    let width = (max_x - min_x).abs().ceil();
    let height = (max_y - min_y).abs().ceil();

    if width == 0.0 || height == 0.0 {
        return None;
    }

    // let scale = (f64::from(target_res) / (width * height * TILE_RES))
    //     .sqrt()
    //     .max(min_scale);

    let scale = ((TILE_RES * width.sqrt() * height.sqrt()) / target_res).max(min_scale);
    let scale = (scale * 4.0).ceil() / 4.0;
    let tile_res = TILE_RES / scale;

    Some(TargetSize::new(
        (width * tile_res).ceil() as u32,
        (height * tile_res).ceil() as u32,
        scale,
        MapPosition::XY { x: min_x, y: min_y },
        MapPosition::XY { x: max_x, y: max_y },
    ))
}

pub fn bp_entity2render_opts(
    value: &blueprint::Entity,
    data: &DataUtil,
) -> prototypes::entity::RenderOpts {
    prototypes::entity::RenderOpts {
        position: (&value.position).into(),
        direction: value.direction,
        orientation: value.orientation,
        variation: value.variation,
        pickup_position: value
            .pickup_position
            .as_ref()
            .map(|v| (f64::from(v.x), f64::from(v.y)).into()),
        connections: None,
        underground_in: value
            .type_
            .as_ref()
            .map(|t| matches!(t, blueprint::UndergroundType::Input)),
        connected_gates: Vec::new(),
        draw_gate_patch: false,
        arithmetic_operation: value.control_behavior.as_ref().and_then(|bhv| {
            bhv.arithmetic_conditions
                .as_ref()
                .map(blueprint::ArithmeticData::operation)
        }),
        decider_operation: value.control_behavior.as_ref().and_then(|bhv| {
            bhv.decider_conditions
                .as_ref()
                .map(blueprint::DeciderData::operation)
        }),
        runtime_tint: value.color.as_ref().map(std::convert::Into::into),
        entity_id: value.entity_number,
        circuit_connected: value.connections.is_some() || !value.neighbours.is_empty(),
        logistic_connected: value
            .control_behavior
            .as_ref()
            .is_some_and(|c| c.connect_to_logistic_network.unwrap_or_default()),
        fluid_recipe: data.recipe_has_fluid(&value.recipe),
    }
}

#[instrument(skip_all, fields(preset, mods))]
pub async fn load_data(
    bp: &blueprint::Data,
    factorio_appdir: &Path,
    factorio_userdir: &Path,
    factorio_bin: &Path,
    preset: Option<preset::Preset>,
    mods: &[String],
    prototype_dump: Option<PathBuf>,
) -> Result<(DataUtil, UsedMods), ScannerError> {
    let bp = bp
        .as_blueprint()
        .ok_or(report!(ScannerError::NoBlueprint))?;

    info!("loaded BP");

    let mut mod_list =
        ModList::generate_custom(factorio_appdir.join("data"), factorio_userdir.join("mods"))
            .change_context(ScannerError::SetupError)?;

    // get used mods from preset or detect from BP meta info
    let mut required_mods = std::iter::once((
        "base".to_owned(),
        DependencyVersion::Exact(prototypes::targeted_engine_version()),
    ))
    .collect::<HashMap<_, _>>();
    required_mods.extend(
        preset
            .as_ref()
            .map_or_else(|| bp_helper::get_used_versions(bp), |p| p.used_mods()),
    );
    required_mods.extend(mods.iter().map(|m| (m.clone(), DependencyVersion::Any)));

    debug!(
        "required mods: {}",
        required_mods
            .iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    );

    if !required_mods.is_empty() {
        debug!("checking mod dependencies");

        let used_mods = resolve_mod_dependencies(&required_mods, &mut mod_list)
            .await
            .change_context(ScannerError::SetupError)?;

        let missing = mod_list.enable_mods(&used_mods);
        if missing.is_empty() {
            debug!("all mods are already installed");
        } else {
            info!("downloading missing mods from mod portal");
            download_mods(missing, &factorio_userdir.join("mods"))
                .await
                .change_context(ScannerError::SetupError)?;
        }
    }

    let active_mods = mod_list.active_mods();
    debug!(
        "{} mods active: {:?}",
        active_mods.len(),
        active_mods.keys().collect::<Vec<_>>()
    );

    let data = if let Some(path) = prototype_dump {
        DataRaw::load(&path).change_context(ScannerError::SetupError)?
    } else {
        get_protodump(
            factorio_userdir,
            factorio_bin,
            &mod_list,
            (
                bp_helper::get_used_startup_settings(bp).unwrap_or(&BTreeMap::new()),
                bp.version,
            ),
        )?
    };

    info!("loaded prototype data");
    Ok((DataUtil::new(data), active_mods))
}

#[instrument(skip_all)]
pub fn render(
    raw_bp: &blueprint::Data,
    data: &DataUtil,
    used_mods: &UsedMods,
    target_res: f64,
    min_scale: f64,
) -> Result<(Vec<u8>, HashSet<String>, Option<Vec<u8>>), ScannerError> {
    let bp = raw_bp
        .as_blueprint()
        .ok_or(report!(ScannerError::NoBlueprint))?;

    let size =
        calculate_target_size(bp, data, target_res, min_scale).ok_or(ScannerError::RenderError)?;
    info!("target size: {size}");

    let image_cache = &mut ImageCache::new();
    let (img, unknown) = render_bp(
        bp,
        data,
        used_mods,
        RenderLayerBuffer::new(size),
        image_cache,
    )
    .ok_or(ScannerError::RenderError)?;
    info!("render completed");

    let mut res = Vec::new();
    let enc = png::PngEncoder::new_with_quality(
        &mut res,
        png::CompressionType::Best,
        png::FilterType::default(),
    );

    enc.write_image(
        img.as_bytes(),
        img.width(),
        img.height(),
        img.color().into(),
    )
    .change_context(ScannerError::RenderError)?;

    let thumbnail = render_thumbnail(raw_bp, data, used_mods, image_cache).map(|t| {
        let mut res = Vec::new();
        let enc = png::PngEncoder::new_with_quality(
            &mut res,
            png::CompressionType::Best,
            png::FilterType::default(),
        );

        let _ = enc.write_image(t.as_bytes(), t.width(), t.height(), t.color().into());
        res
    });

    Ok((res, unknown, thumbnail))
}

#[instrument(skip_all)]
#[allow(clippy::too_many_lines)]
pub fn render_bp(
    bp: &blueprint::Blueprint,
    data: &prototypes::DataUtil,
    used_mods: &UsedMods,
    mut render_layers: RenderLayerBuffer,
    image_cache: &mut ImageCache,
) -> Option<(image::DynamicImage, HashSet<String>)> {
    let mut unknown = HashSet::new();
    let mut wire_connections = EntityWireConnections::new();
    let mut pipe_connections = HashMap::<MapPosition, HashSet<Direction>>::new();
    let mut heat_connections = HashMap::<MapPosition, HashSet<Direction>>::new();

    let Some(util_sprites) = data.util_sprites() else {
        warn!("failed to load util sprites, required for wire rendering & alt mode");
        return None;
    };

    let Some(indicator_arrow) = util_sprites.indication_arrow.render(
        render_layers.scale() * 1.25,
        used_mods,
        image_cache,
        &SimpleGraphicsRenderOpts::default(),
    ) else {
        warn!("failed to load indicator arrow sprite, required for alt mode");
        return None;
    };

    let Some(indicator_line) = util_sprites.indication_line.render(
        render_layers.scale() * 1.25,
        used_mods,
        image_cache,
        &SimpleGraphicsRenderOpts::default(),
    ) else {
        warn!("failed to load indicator line sprite, required for alt mode");
        return None;
    };

    // pipe / heat connections
    bp.entities.iter().for_each(|e| {
        let Some(e_data) = data.get_entity(&e.name) else {
            return;
        };

        let options = bp_entity2render_opts(e, data);
        e_data
            .pipe_connections(&options)
            .iter()
            .copied()
            .for_each(|(pos, dir)| {
                pipe_connections.entry(pos).or_default().insert(dir);
            });
        e_data
            .heat_connections(&options)
            .iter()
            .copied()
            .for_each(|(pos, dir)| {
                heat_connections.entry(pos).or_default().insert(dir);
            });
    });

    // render entities
    let rendered_count = bp
        .entities
        .iter()
        .filter_map(|e| {
            let Some(e_data) = data.get_entity(&e.name) else {
                unknown.insert((*e.name).clone());
                return None;
            };

            let mut connected_gates: Vec<Direction> = Vec::new();
            let mut draw_gate_patch = false;
            let connections = data.get_entity_type(&e.name).and_then(|entity_type| {
                if entity_type.connectable() {
                    let mut up = false;
                    let mut down = false;
                    let mut left = false;
                    let mut right = false;

                    let pos: types::MapPosition = (&e.position).into();

                    match entity_type {
                        EntityType::Pipe | EntityType::InfinityPipe | EntityType::PipeToGround => {
                            for (p, dirs) in &pipe_connections {
                                if p.is_close(&pos, 0.5) {
                                    for dir in dirs {
                                        match dir {
                                            Direction::North => up = true,
                                            Direction::South => down = true,
                                            Direction::East => right = true,
                                            Direction::West => left = true,
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        EntityType::HeatPipe | EntityType::HeatInterface => {
                            for (p, dirs) in &heat_connections {
                                if p.is_close(&pos, 0.5) {
                                    for dir in dirs {
                                        match dir {
                                            Direction::North => up = true,
                                            Direction::South => down = true,
                                            Direction::East => right = true,
                                            Direction::West => left = true,
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            for other in &bp.entities {
                                if other == e {
                                    continue;
                                }

                                let Some(other_type) = data.get_entity_type(&other.name) else {
                                    continue;
                                };

                                if !entity_type.can_connect_to(other_type) {
                                    continue;
                                }

                                if matches!(entity_type, EntityType::Wall)
                                    && matches!(other_type, EntityType::Wall)
                                {
                                    let Some(src) = data
                                        .get_proto::<WallPrototype>(&e.name)
                                        .map(|p| p.visual_merge_group)
                                    else {
                                        continue;
                                    };

                                    let Some(dst) = data
                                        .get_proto::<WallPrototype>(&other.name)
                                        .map(|p| p.visual_merge_group)
                                    else {
                                        continue;
                                    };

                                    if src != dst {
                                        continue;
                                    }
                                }

                                let other_pos: types::MapPosition = (&other.position).into();

                                match entity_type {
                                    EntityType::Gate => {
                                        match pos.is_cardinal_neighbor(&other_pos) {
                                            Some(dir) => {
                                                if dir == Direction::South {
                                                    draw_gate_patch = true;
                                                }
                                            }
                                            None => continue,
                                        }
                                    }
                                    EntityType::Wall => {
                                        match pos.is_cardinal_neighbor(&other_pos) {
                                            Some(dir) => {
                                                if matches!(other_type, EntityType::Gate) {
                                                    if dir.is_straight(&other.direction) {
                                                        connected_gates.push(dir);
                                                    }
                                                } else {
                                                    match dir {
                                                        Direction::North => up = true,
                                                        Direction::South => down = true,
                                                        Direction::East => right = true,
                                                        Direction::West => left = true,
                                                        _ => continue,
                                                    }
                                                }
                                            }
                                            None => continue,
                                        }
                                    }
                                    EntityType::TransportBelt => {
                                        let neighbor = match other_type {
                                            EntityType::TransportBelt => {
                                                pos.is_cardinal_neighbor(&other_pos)
                                            }
                                            EntityType::UndergroundBelt
                                            | EntityType::LinkedBelt => {
                                                let dir = pos.is_cardinal_neighbor(&other_pos);

                                                if let Some(dir) = dir {
                                                    let Some(u_output) =
                                                        other.type_.as_ref().map(|t| {
                                                            matches!(
                                                                t,
                                                                blueprint::UndergroundType::Output
                                                            )
                                                        })
                                                    else {
                                                        continue;
                                                    };

                                                    let other_dir = if u_output {
                                                        other.direction.flip()
                                                    } else {
                                                        other.direction
                                                    };

                                                    if dir != other_dir {
                                                        continue;
                                                    }
                                                }

                                                dir
                                            }
                                            EntityType::Splitter => {
                                                pos.is_2wide_cardinal_neighbor(&other_pos)
                                            }
                                            EntityType::Loader => {
                                                pos.is_2long_cardinal_neighbor(&other_pos)
                                            }
                                            _ => continue,
                                        };

                                        if let Some(dir) = neighbor {
                                            if dir != other.direction.flip() {
                                                continue;
                                            }

                                            match dir {
                                                Direction::North => up = true,
                                                Direction::South => down = true,
                                                Direction::East => right = true,
                                                Direction::West => left = true,
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }

                    Some(ConnectedDirections::from_directions(up, down, left, right))
                } else {
                    None
                }
            });

            let mut render_opts = bp_entity2render_opts(e, data);
            render_opts.connections = connections;
            render_opts.connected_gates = connected_gates;
            render_opts.draw_gate_patch = draw_gate_patch;

            'recipe_icon: {
                if !e.recipe.is_empty() && e_data.recipe_visible() {
                    if !data.contains_recipe(&e.recipe) {
                        unknown.insert((*e.recipe).clone());
                        break 'recipe_icon;
                    }

                    if let Some(icon) = data.get_recipe_icon(
                        &e.recipe,
                        render_layers.scale() * 0.75,
                        used_mods,
                        image_cache,
                    ) {
                        render_layers.add(
                            icon,
                            &render_opts.position,
                            InternalRenderLayer::IconOverlay,
                        );
                    } else {
                        warn!(
                            "failed to render recipe icon for {} at {:?} [{}]",
                            e.recipe, e.position, e.name
                        );
                    }
                }
            }

            // filter icons / priority arrows
            'filters_priority: {
                if let Some(prio_in) = &e.input_priority {
                    let offset = e.direction.rotate_vector(
                        prio_in.as_vector() + Vector::Tuple(0.0, 0.25) + indicator_arrow.1,
                    );

                    let arrow = match e.direction {
                        Direction::North => indicator_arrow.0.clone(),
                        Direction::East => imageops::rotate90(&indicator_arrow.0).into(),
                        Direction::South => imageops::rotate180(&indicator_arrow.0).into(),
                        Direction::West => imageops::rotate270(&indicator_arrow.0).into(),
                        _ => break 'filters_priority,
                    };

                    render_layers.add(
                        (arrow, offset),
                        &render_opts.position,
                        InternalRenderLayer::DirectionOverlay,
                    );
                }

                if let Some(prio_out) = &e.output_priority {
                    if e.filter.is_empty() {
                        let offset = e.direction.rotate_vector(
                            prio_out.as_vector() + Vector::Tuple(0.0, -0.25) + indicator_arrow.1,
                        );

                        let arrow = match e.direction {
                            Direction::North => indicator_arrow.0.clone(),
                            Direction::East => imageops::rotate90(&indicator_arrow.0).into(),
                            Direction::South => imageops::rotate180(&indicator_arrow.0).into(),
                            Direction::West => imageops::rotate270(&indicator_arrow.0).into(),
                            _ => break 'filters_priority,
                        };

                        render_layers.add(
                            (arrow, offset),
                            &render_opts.position,
                            InternalRenderLayer::DirectionOverlay,
                        );
                    } else {
                        let Some(filter) = data.get_item_icon(
                            &e.filter,
                            render_layers.scale() * 2.2,
                            used_mods,
                            image_cache,
                        ) else {
                            warn!(
                                "failed to render filter icon for {} at {:?} [{}]",
                                e.filter, e.position, e.name
                            );
                            break 'filters_priority;
                        };

                        let offset = e.direction.rotate_vector(prio_out.as_vector() + filter.1);

                        render_layers.add(
                            (filter.0, offset),
                            &render_opts.position,
                            InternalRenderLayer::IconOverlay,
                        );
                    }
                }

                if !e.filters.is_empty() {
                    let filter_count = e.filters.len();
                    let mut offset = if filter_count == 1 {
                        Vector::Tuple(0.0, 0.0)
                    } else if filter_count == 2 {
                        Vector::Tuple(-0.25, 0.0)
                    } else {
                        Vector::Tuple(-0.25, -0.25)
                    };

                    for idx in 0..filter_count.min(4) {
                        if idx == 2 {
                            offset += Vector::Tuple(-1.0, 0.5);
                        }

                        let Some(filter) = data.get_item_icon(
                            &e.filters[idx],
                            render_layers.scale() * 2.2,
                            used_mods,
                            image_cache,
                        ) else {
                            warn!(
                                "failed to render filter icon for {} at {:?} [{}]",
                                e.filters[idx], e.position, e.name
                            );
                            continue;
                        };

                        render_layers.add(
                            (filter.0, filter.1 + offset),
                            &render_opts.position,
                            InternalRenderLayer::IconOverlay,
                        );

                        offset += Vector::Tuple(0.5, 0.0);
                    }
                }
            }

            // modules / item requests
            {
                if !e.items.is_empty() {
                    let mut items = e.items.iter().collect::<Vec<_>>();
                    items.sort_unstable_by_key(|a| a.0);

                    let scale = render_layers.scale() * 2.3;
                    let s_box = e_data.selection_box();
                    let width = s_box.width() - 0.25;
                    let height = s_box.height();
                    let count = items.iter().map(|(_, &c)| c).sum::<u32>();

                    let row_len = (width / 0.5).floor() as u32;
                    let row_count = (f64::from(count) / f64::from(row_len)).ceil() as u32;
                    let row_len = (f64::from(count) / f64::from(row_count)).ceil() as u32;

                    let start_y =
                        ((height / 4.0) - (f64::from(row_count - 1) / 2.0) + 0.25).max(0.0);
                    let mut offset = Vector::Tuple(0.0, start_y);

                    let icons = items
                        .iter()
                        .filter_map(|(name, _)| {
                            Some((
                                (*name).clone(),
                                data.get_item_icon(name, scale, used_mods, image_cache)?,
                            ))
                        })
                        .collect::<HashMap<_, _>>();

                    for chunk in e
                        .items
                        .iter()
                        .flat_map(|(i, c)| std::iter::repeat(i).take(*c as usize))
                        .collect::<Vec<_>>()
                        .as_slice()
                        .chunks(row_len as usize)
                    {
                        let count = chunk.len() as u32;
                        if count == 0 {
                            continue;
                        }

                        let start_x = f64::from(count - 1) * -0.25; // count / 2 * -0.5
                        offset += Vector::Tuple(start_x, 0.0);

                        for &item in chunk {
                            if let Some(icon) = icons.get(item) {
                                render_layers.add(
                                    (icon.0.clone(), offset),
                                    &render_opts.position,
                                    InternalRenderLayer::IconOverlay,
                                );
                            }

                            offset += Vector::Tuple(0.5, 0.0);
                        }

                        offset = Vector::Tuple(0.0, offset.y() + 0.5);
                    }
                }
            }

            // inserter indicators
            'inserter_indicators: {
                let Some(proto) = data.get_proto::<InserterPrototype>(&e.name) else {
                    break 'inserter_indicators;
                };

                #[allow(clippy::items_after_statements)]
                fn indicator_helper(
                    pos: Vector,
                    opts: &prototypes::entity::RenderOpts,
                    graphics: &(image::DynamicImage, Vector),
                    layers: &mut RenderLayerBuffer,
                ) {
                    let img = if pos.x() != 0.0 && pos.x() != 0.0 {
                        let angle = pos.y().atan2(pos.x()) + std::f64::consts::FRAC_PI_2;
                        rotate_about_center(
                            &graphics.0.to_rgba8(),
                            angle as f32,
                            geometric_transformations::Interpolation::Nearest,
                            image::Rgba([0, 0, 0, 0]),
                        )
                        .into()
                    } else if pos.y() < 0.0 {
                        graphics.0.clone()
                    } else if pos.y() > 0.0 {
                        imageops::rotate180(&graphics.0).into()
                    } else if pos.x() > 0.0 {
                        imageops::rotate90(&graphics.0).into()
                    } else {
                        imageops::rotate270(&graphics.0).into()
                    };

                    layers.add(
                        (img, pos.shorten_by(0.45)),
                        &opts.position,
                        InternalRenderLayer::DirectionOverlay,
                    );
                }

                indicator_helper(
                    proto.get_pickup_position(
                        e.direction,
                        e.pickup_position.as_ref().map(std::convert::Into::into),
                    ),
                    &render_opts,
                    &indicator_line,
                    &mut render_layers,
                );
                indicator_helper(
                    proto.get_insert_position(
                        e.direction,
                        e.drop_position.as_ref().map(std::convert::Into::into),
                    ),
                    &render_opts,
                    &indicator_arrow,
                    &mut render_layers,
                );
            }

            // store wire connections for wire rendering
            let mut wires0 = e
                .neighbours
                .iter()
                .map(|n| (*n, [true, false, false]))
                .collect::<ConnectedEntities>();
            let mut wires1 = ConnectedEntities::new();
            let mut wires2 = ConnectedEntities::new();
            let mut is_switch = false;

            if let Some(circuit_cons) = &e.connections {
                match circuit_cons {
                    blueprint::Connection::SingleOne { one } => one.transform(&mut wires0),
                    blueprint::Connection::SingleTwo { two } => two.transform(&mut wires1),
                    blueprint::Connection::Double { one, two } => {
                        one.transform(&mut wires0);
                        two.transform(&mut wires1);
                    }
                    blueprint::Connection::Switch { one, cu0, cu1 } => {
                        one.transform(&mut wires0);
                        cu0.transform(&mut wires1);
                        cu1.transform(&mut wires2);
                        is_switch = true;
                    }
                }
            }

            if !wires0.is_empty() || !wires1.is_empty() | !wires2.is_empty() {
                wire_connections.insert(
                    e.entity_number,
                    (
                        e.position.clone().into(),
                        ([wires0, wires1, wires2], is_switch),
                    ),
                );
            }

            data.render_entity(
                &e.name,
                &render_opts,
                used_mods,
                &mut render_layers,
                image_cache,
            )
        })
        .count();

    info!("entities: {}, layers: {rendered_count}", bp.entities.len());

    // render tiles
    let rendered_count = bp
        .tiles
        .iter()
        .filter_map(|t| {
            let Some(tile) = data.get_proto::<TilePrototype>(&t.name) else {
                unknown.insert((*t.name).clone());
                return None;
            };

            let position: MapPosition = (&t.position).into();
            tile.render(
                &(position + MapPosition::Tuple(0.5, 0.5)),
                used_mods,
                &mut render_layers,
                image_cache,
            )
        })
        .count();

    info!("tiles: {}, layers: {rendered_count}", bp.tiles.len());

    render_layers.draw_wires(&wire_connections, util_sprites, used_mods, image_cache);
    render_layers.generate_background();

    Some((render_layers.combine(), unknown))
}

#[instrument(skip_all)]
pub fn render_thumbnail(
    bp: &blueprint::Data,
    data: &prototypes::DataUtil,
    used_mods: &UsedMods,
    image_cache: &mut ImageCache,
) -> Option<image::DynamicImage> {
    static BASE_SCALE: f64 = 0.125;
    let size = (32.0 / BASE_SCALE).round() as u32;

    let mut layers = RenderLayerBuffer::new(TargetSize::new(
        size,
        size,
        BASE_SCALE,
        MapPosition::Tuple(-1.0, -1.0),
        MapPosition::Tuple(1.0, 1.0),
    ));

    layers.add(
        (
            data.get_item_icon(bp.item(), BASE_SCALE, used_mods, image_cache)?
                .0,
            Vector::Tuple(-0.5, -0.5),
        ),
        &MapPosition::default(),
        InternalRenderLayer::Entity,
    );

    let icons = bp.icons();
    if icons.is_empty() {
        return Some(layers.combine());
    }

    let icon_count = icons.len();
    let (scale, s_x, s_y) = if icon_count == 1 {
        (BASE_SCALE * 1.2, -0.5, -0.5)
    } else if icon_count == 2 {
        (BASE_SCALE * 2.2, -0.75, -0.5)
    } else {
        (BASE_SCALE * 2.2, -0.75, -0.75)
    };

    let mut offset = Vector::Tuple(s_x, s_y);

    icons
        .iter()
        .enumerate()
        .take(icon_count.min(4))
        .for_each(|(idx, icon)| {
            if idx == 2 {
                offset += Vector::Tuple(-1.0, 0.5);
            }

            let res = match &icon.signal {
                SignalID::Item { name } => data.get_item_icon(
                    name.clone().unwrap_or_default().as_str(),
                    scale,
                    used_mods,
                    image_cache,
                ),
                SignalID::Fluid { name } => data.get_fluid_icon(
                    name.clone().unwrap_or_default().as_str(),
                    scale,
                    used_mods,
                    image_cache,
                ),
                SignalID::Virtual { name } => data.get_signal_icon(
                    name.clone().unwrap_or_default().as_str(),
                    scale,
                    used_mods,
                    image_cache,
                ),
            };

            let Some((res, _)) = res else {
                return;
            };

            layers.add(
                (res, offset),
                &MapPosition::default(),
                InternalRenderLayer::AboveEntity,
            );

            offset += Vector::Tuple(0.5, 0.0);
        });

    Some(layers.combine())
}

#[derive(Debug, thiserror::Error)]
pub enum PlayerDataError {
    #[error("failed to load player data: {0}")]
    Load(#[from] std::io::Error),

    #[error("failed to parse player data: {0}")]
    Parse(#[from] serde_json::Error),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "service-username")]
    username: Option<String>,

    #[serde(rename = "service-token")]
    token: Option<String>,
}

impl PlayerData {
    pub fn load(path: &Path) -> std::result::Result<Self, PlayerDataError> {
        let mut bytes = Vec::new();
        fs::File::open(path)?.read_to_end(&mut bytes)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug)]
pub struct DependencyResolutionError;

impl Context for DependencyResolutionError {}

impl std::fmt::Display for DependencyResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod dependency resolving error")
    }
}

#[instrument(skip_all, fields(required = required.keys().cloned().collect::<Vec<_>>().join(", ")))]
pub async fn resolve_mod_dependencies(
    required: &DependencyList,
    mod_list: &mut ModList,
) -> Result<UsedVersions, DependencyResolutionError> {
    // load local dependency info of required mods and their dependencies
    {
        let span = info_span!("load_local_deps", loaded_mods = field::Empty).entered();
        let mut queue = required
            .iter()
            .map(|(n, d)| (n.clone(), *d))
            .collect::<Vec<_>>();
        let mut completed = HashSet::new();

        while let Some((name, dep_version)) = &queue.pop() {
            if completed.contains(name) {
                continue;
            }

            completed.insert(name.clone());

            if let Some(deps) = mod_list.load_local_dependency_info(name, dep_version) {
                for (dep_name, dep) in deps {
                    if !dep.is_required() {
                        continue;
                    }

                    if !completed.contains(&dep_name) {
                        queue.push((dep_name, *dep.version()));
                    }
                }
            }
        }

        span.record("loaded_mods", completed.len());
        span.exit();
    }

    // try to resolve dependencies with local mods
    match mod_list
        .solve_dependencies(required)
        .change_context(DependencyResolutionError)
        .attach_printable_lazy(|| "could not resolve dependencies with local mods")
    {
        Ok(res) => return Ok(res),
        Err(err) => info!("{err:?}"),
    }

    info!("fetching dependency info from mod portal");

    let mut process_queue = required.keys().cloned().collect::<Vec<_>>();
    let mut fetched_deps = Vec::new();
    fetched_deps.extend(Mod::wube_mods().map(std::string::ToString::to_string));

    while let Some(name) = process_queue.pop() {
        if fetched_deps.contains(&name) {
            continue;
        }

        let info = factorio_api::full_info(&name)
            .await
            .change_context(DependencyResolutionError)
            .attach_printable_lazy(|| format!("fetching mod info for {name} failed"))?;

        let deps_info = info
            .releases
            .into_iter()
            .map(|r| (r.version, r.info_json.dependencies))
            .collect::<HashMap<_, _>>();

        mod_list.set_dependency_info(&name.clone(), deps_info.clone());

        let queue_add = deps_info
            .values()
            .flatten()
            .filter_map(|d| {
                if d.is_required() {
                    Some(d.name().clone())
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        debug!("fetched dependency info for {name}");

        process_queue.extend(queue_add);
        fetched_deps.push(name);
    }

    info!("collected dependency info for {} mods", fetched_deps.len());

    mod_list
        .solve_dependencies(required)
        .change_context(DependencyResolutionError)
}

#[derive(Debug)]
pub enum ModDownloadError {
    MissingCredentials,
    TriedToDownloadWubeMod(String, Version),
    DownloadFailed(String, Version),
    SaveFailed(String, Version),
}

impl Context for ModDownloadError {}

impl std::fmt::Display for ModDownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => {
                write!(f, "missing credentials for mod portal")
            }
            Self::TriedToDownloadWubeMod(name, version) => {
                write!(f, "tried to download wube mod {name} v{version}")
            }
            Self::DownloadFailed(name, version) => {
                write!(f, "failed to download mod {name} v{version}")
            }
            Self::SaveFailed(name, version) => write!(f, "failed to save mod {name} v{version}",),
        }
    }
}

#[instrument(skip_all, fields(count = missing.len()))]
pub async fn download_mods(
    missing: UsedVersions,
    destination: &Path,
) -> Result<(), ModDownloadError> {
    let (username, token) = {
        let env_username = env::var("FACTORIO_USERNAME").ok();
        let env_token = env::var("FACTORIO_TOKEN").ok();

        if let (Some(username), Some(token)) = (env_username.clone(), env_token.clone()) {
            (username, token)
        } else {
            let player_data = PlayerData::load(&destination.join("../player-data.json"))
                .change_context(ModDownloadError::MissingCredentials).attach_printable("you can either use the game to login to your account\nor you provide the environment variables FACTORIO_USERNAME & FACTORIO_TOKEN\nwhich also work from a .env file")?;

            match (
                player_data.username,
                player_data.token,
                env_username,
                env_token,
            ) {
                (Some(username), Some(token), _, _)
                | (Some(username), None, _, Some(token))
                | (None, Some(token), Some(username), _) => (username, token),
                _ => return Err(report!(ModDownloadError::MissingCredentials).attach_printable("you can either use the game to login to your account\nor you provide the environment variables FACTORIO_USERNAME & FACTORIO_TOKEN\nwhich also work from a .env file"))
            }
        }
    };

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));

    for (name, version) in missing {
        ensure!(
            !Mod::wube_mods().contains(&name.as_str()),
            ModDownloadError::TriedToDownloadWubeMod(name, version)
        );

        info!("downloading {name} v{version}");
        let dl = factorio_api::fetch_mod(&name, &version, &username, &token)
            .await
            .change_context(ModDownloadError::DownloadFailed(name.clone(), version))?;

        fs::write(destination.join(format!("{name}_{version}.zip")), dl)
            .change_context(ModDownloadError::SaveFailed(name, version))?;

        interval.tick().await;
    }

    Ok(())
}
