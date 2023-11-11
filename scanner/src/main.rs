#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use mod_util::{
    mod_list::ModList, mod_loader::Mod, mod_settings::SettingsDat, UsedMods, UsedVersions,
};
use prototypes::{DataRaw, DataUtil, EntityType, RenderLayerBuffer, TargetSize};
use types::{
    merge_renders, ConnectedDirections, Direction, GraphicsOutput, ImageCache, MapPosition,
};

mod bp_helper;
mod preset;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file that contains your blueprint string
    #[clap(short, long, value_parser)]
    blueprint: PathBuf,

    /// Path to the factorio directory that contains the data folder (path.read-data)
    #[clap(short, long, value_parser)]
    factorio: PathBuf,

    /// Path to the factorio binary instead of the default expected one
    #[clap(long, value_parser)]
    factorio_bin: Option<PathBuf>,

    /// Path to the data dump json file. If not set, the data will be dumped automatically
    #[clap(long, value_parser)]
    prototype_dump: Option<PathBuf>,

    /// Preset to use
    #[clap(long, value_enum)]
    preset: Option<preset::Preset>,

    /// Path to the output file
    #[clap(short, long, value_parser)]
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let bp = blueprint::Data::try_from(fs::read_to_string(cli.blueprint).unwrap()).unwrap();
    let bp = bp.as_blueprint().unwrap();

    println!("loaded BP");

    let mut mod_list = ModList::generate(&cli.factorio).unwrap();

    // get used mods from preset or detect from BP meta info
    let required_mods = cli
        .preset
        .as_ref()
        .map_or(bp_helper::get_used_versions(bp).unwrap_or_default(), |p| {
            p.used_mods()
        });

    if !required_mods.is_empty() {
        println!("checking mod dependencies");

        let used_mods = match resolve_mod_dependencies(&required_mods, &mut mod_list) {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        };

        let missing = mod_list.enable_mods(&used_mods);
        if missing.is_empty() {
            println!("all mods are already installed");
        } else {
            println!("downloading missing mods from mod portal");
            download_mods(missing, &cli.factorio).unwrap();
        }
    }

    let used_mods = mod_list.active_mods();

    let data_raw = if let Some(path) = cli.prototype_dump {
        DataRaw::load(&path).unwrap()
    } else {
        mod_list.save().unwrap();
        println!("updated mod-list.json");

        // set settings used in BP
        let empty_settings = &HashMap::new();
        let settings =
            bp_helper::get_used_startup_settings(bp).map_or(empty_settings, |settings| settings);
        SettingsDat::load_bp_settings(
            settings,
            bp.info.version,
            &cli.factorio.join("mods/mod-settings.dat"),
        )
        .unwrap()
        .save()
        .unwrap();

        println!("updated mod-settings.dat");

        // execute factorio to dump prototypes
        print!("dumping prototypes ");
        std::io::stdout().flush().unwrap();
        let binary_path = cli.factorio.join("bin/x64/run");
        let dump_out = Command::new(cli.factorio_bin.unwrap_or(binary_path))
            .arg("--dump-data")
            .output()
            .expect("failed to execute process");

        if dump_out.status.success() {
            println!("success");
        } else {
            println!("failed!");
            println!("{}", String::from_utf8_lossy(&dump_out.stderr));
            panic!("failed to dump prototypes");
        }

        let dump_path = cli.factorio.join("script-output/data-raw-dump.json");
        DataRaw::load(&dump_path).unwrap()
    };

    println!("loaded prototype data");

    // =====[  RENDER TEST  ]=====
    let data = DataUtil::new(data_raw);

    // render every entity once and check if it is empty or not
    // let mut image_cache = ImageCache::new();
    // println!("mods: {used_mods:?}");
    // for name in data.entities() {
    //     render_by_name(
    //         name, //"se-spaceship-rocket-booster-tank",
    //         &data,
    //         &EntityRenderOpts {
    //             factorio_dir: &cli.factorio,
    //             used_mods: used_mods.clone(),
    //             ..Default::default()
    //         },
    //         &mut image_cache,
    //     );
    // }

    let active_mods = mod_list.active_mods();

    println!(
        "{} mods active: {:?}",
        active_mods.len(),
        active_mods.keys().collect::<Vec<_>>()
    );

    let size = calculate_target_size(&bp, &data, 2048.0, 0.5).unwrap();
    println!("target size: {size:?}");

    let mut layer_buff = RenderLayerBuffer::new(size);

    match render_bp(
        bp,
        &data,
        &active_mods,
        &mut layer_buff,
        &mut ImageCache::new(),
    ) {
        Some((img, scale, shift)) => {
            println!("render done");

            let shrink_factor = (1.0 / scale).round() as u32;

            let img = img.resize(
                img.dimensions().0 / shrink_factor,
                img.dimensions().1 / shrink_factor,
                image::imageops::FilterType::CatmullRom,
            );
            // println!(
            //     "{}x{} x{scale} {shift}",
            //     img.dimensions().0,
            //     img.dimensions().1,
            // );

            img.save(&cli.out).unwrap();
            println!("saved to {:?}", cli.out);
        }
        None => println!("EMPTY BP!"),
    }
}

fn calculate_target_size(
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

    let mut unknown = HashSet::new();

    for entity in &bp.entities {
        let Some(e_proto) = data.get_entity(&entity.name) else {
            unknown.insert(entity.name.as_str());
            continue;
        };

        let e_pos: MapPosition = (&entity.position).into();
        let c_box = e_proto.collision_box();

        let tl = &e_pos + c_box.top_left();
        let br = &e_pos + c_box.bottom_right();

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

    // for tile in &bp.tiles {
    //     let Some(t_proto) = data.get_tile(&tile.name) else {
    //         unknown.insert(tile.name.as_str());
    //         continue;
    //     };
    // }

    if !unknown.is_empty() {
        println!("unknown entities: {unknown:?}");
    }

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

fn bp_entity2render_opts(value: &blueprint::Entity) -> prototypes::EntityRenderOpts {
    prototypes::EntityRenderOpts {
        position: (&value.position).into(),
        direction: value.direction,
        orientation: value.orientation.map(f64::from),
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
    }
}

#[allow(clippy::too_many_lines)]
fn render_bp(
    bp: &blueprint::Blueprint,
    data: &prototypes::DataUtil,
    used_mods: &UsedMods,
    render_layers: &mut RenderLayerBuffer,
    image_cache: &mut ImageCache,
) -> Option<GraphicsOutput> {
    let renders = bp
        .entities
        .iter()
        .filter_map(|e| {
            let mut connected_gates: Vec<Direction> = Vec::new();
            let mut draw_gate_patch = false;
            let connections = data.get_type(&e.name).and_then(|entity_type| {
                if entity_type.connectable() {
                    let mut up = false;
                    let mut down = false;
                    let mut left = false;
                    let mut right = false;

                    let pos: types::MapPosition = (&e.position).into();

                    for other in &bp.entities {
                        if other == e {
                            continue;
                        }

                        let Some(other_type) = data.get_type(&other.name) else {
                            continue;
                        };

                        if !entity_type.can_connect_to(other_type) {
                            continue;
                        }

                        let other_pos: types::MapPosition = (&other.position).into();

                        match entity_type {
                            EntityType::Gate => match pos.is_cardinal_neighbor(&other_pos) {
                                Some(dir) => {
                                    if dir == Direction::South {
                                        draw_gate_patch = true;
                                    }
                                }
                                None => continue,
                            },
                            EntityType::Wall => match pos.is_cardinal_neighbor(&other_pos) {
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
                            },
                            EntityType::Pipe | EntityType::InfinityPipe => {
                                if !matches!(
                                    &other_type,
                                    EntityType::Pipe
                                        | EntityType::InfinityPipe
                                        | EntityType::PipeToGround
                                ) {
                                    continue;
                                }

                                if let Some(dir) = pos.is_cardinal_neighbor(&other_pos) {
                                    if matches!(other_type, EntityType::PipeToGround)
                                        && dir != other.direction.flip()
                                    {
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
                            EntityType::HeatPipe | EntityType::HeatInterface => {
                                if !matches!(
                                    &other_type,
                                    EntityType::HeatPipe | EntityType::HeatInterface
                                ) {
                                    continue;
                                }

                                if let Some(dir) = pos.is_cardinal_neighbor(&other_pos) {
                                    match dir {
                                        Direction::North => up = true,
                                        Direction::South => down = true,
                                        Direction::East => right = true,
                                        Direction::West => left = true,
                                        _ => {}
                                    }
                                }
                            }
                            EntityType::TransportBelt => {
                                let neighbor = match other_type {
                                    EntityType::TransportBelt
                                    | EntityType::UndergroundBelt
                                    | EntityType::LinkedBelt => {
                                        pos.is_cardinal_neighbor(&other_pos)
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

                    Some(ConnectedDirections::from_directions(up, down, left, right))
                } else {
                    None
                }
            });

            let mut render_opts = bp_entity2render_opts(e);
            render_opts.connections = connections;
            render_opts.connected_gates = connected_gates;
            render_opts.draw_gate_patch = draw_gate_patch;

            data.render_entity(&e.name, &render_opts, used_mods, render_layers, image_cache)
                .map(|(img, scale, shift)| {
                    let (shift_x, shift_y) = shift.as_tuple();
                    Some((
                        img,
                        scale,
                        (
                            shift_x + f64::from(e.position.x),
                            shift_y + f64::from(e.position.y),
                        )
                            .into(),
                    ))
                })
        })
        .collect::<Vec<_>>();

    println!("entities: {}, layers: {}", bp.entities.len(), renders.len());

    merge_renders(renders.as_slice())
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayerData {
    #[serde(rename = "service-username")]
    username: Option<String>,

    #[serde(rename = "service-token")]
    token: Option<String>,
}

impl PlayerData {
    pub fn load(path: &Path) -> Option<Self> {
        let mut bytes = Vec::new();
        fs::File::open(path).ok()?.read_to_end(&mut bytes).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}

fn resolve_mod_dependencies(
    required: &UsedVersions,
    mod_list: &mut ModList,
) -> anyhow::Result<UsedVersions> {
    if let Ok(res) = mod_list.solve_dependencies(required) {
        return Ok(res);
    }

    println!(
        "Could not resolve dependencies with local mods, fetching dependency info from mod portal"
    );

    let mut process_queue = required.keys().cloned().collect::<Vec<_>>();
    let mut fetched_deps = Vec::new();
    fetched_deps.extend(Mod::wube_mods().map(std::string::ToString::to_string));

    while let Some(name) = process_queue.pop() {
        if fetched_deps.contains(&name) {
            continue;
        }

        print!("fetching mod info for {name}: ");
        std::io::stdout().flush().unwrap();
        let Some(info) = factorio_api::blocking::full_info(&name) else {
            println!("failed");
            continue;
        };
        println!("done");

        let deps_info = info
            .releases
            .into_iter()
            .map(|r| (r.version, r.info_json.dependencies))
            .collect::<HashMap<_, _>>();

        mod_list.set_dependency_info(&name, deps_info.clone());

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

        process_queue.extend(queue_add);
        fetched_deps.push(name);
    }

    mod_list.solve_dependencies(required)
}

fn download_mods(missing: UsedVersions, factorio_dir: &Path) -> anyhow::Result<()> {
    let mods_path = factorio_dir.join("mods");
    let Some(player_data) = PlayerData::load(&factorio_dir.join("player-data.json")) else {
        panic!("You need to login inside factorio first!");
    };

    let Some(username) = player_data.username else {
        panic!("You need to login inside factorio first!");
    };

    let Some(token) = player_data.token else {
        panic!("You need to login inside factorio first!");
    };

    for (name, version) in missing {
        if Mod::wube_mods().contains(&name.as_str()) {
            return Err(anyhow::anyhow!(
                "Tried to download {name} v{version}, a wube mod! You need to install it manually!"
            ));
        }

        print!("downloading {name} v{version}: ");
        std::io::stdout().flush().unwrap();
        let Some(dl) = factorio_api::blocking::fetch_mod(&name, &version, &username, &token) else {
            println!("failed");
            return Err(anyhow::anyhow!("failed to download mod"));
        };

        match fs::write(mods_path.join(format!("{name}_{version}.zip")), dl) {
            Ok(()) => println!("done"),
            Err(e) => {
                println!("failed -> {e}");
                return Err(anyhow::anyhow!("failed to write mod"));
            }
        }
    }

    Ok(())
}
