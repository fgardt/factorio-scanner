#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use clap::Parser;
use error_stack::{ensure, report, Context, Result, ResultExt};
use image::{codecs::png, ImageEncoder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[macro_use]
extern crate log;

use mod_util::{
    mod_info::Version, mod_list::ModList, mod_loader::Mod, mod_settings::SettingsDat, UsedMods,
    UsedVersions,
};
use prototypes::{entity::Type as EntityType, InternalRenderLayer};
use prototypes::{DataRaw, DataUtil, RenderLayerBuffer, TargetSize};
use types::{ConnectedDirections, Direction, ImageCache, MapPosition};

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

    /// Target resolution (1 side of a square) in pixels
    #[clap(long = "res", default_value_t = 2048.0)]
    target_res: f64,

    /// Minimum scale to use (below 0.5 makes not much sense, vanilla HR mode is 0.5)
    #[clap(long, default_value_t = 0.5)]
    min_scale: f64,

    /// Sets the used logging level
    /// Possible values: error, warn, info, debug, trace
    /// For no logging don't set this option
    /// Note: the LOG_LEVEL environment variable overrides this option
    #[clap(long, value_parser, verbatim_doc_comment)]
    log_level: Option<log::Level>,
}

#[derive(Debug)]
enum ScannerError {
    SetupError,
    RenderError,
    NoBlueprint,
}

impl Context for ScannerError {}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetupError => write!(f, "setup error"),
            Self::RenderError => write!(f, "render error"),
            Self::NoBlueprint => write!(f, "no blueprint"),
        }
    }
}

fn main() -> ExitCode {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let level = cli
        .log_level
        .as_ref()
        .map_or("info", |level| level.as_str());
    if let Err(logger_err) = pretty_logging::init(level) {
        eprintln!("{logger_err:?}");
        return ExitCode::FAILURE;
    };

    if let Err(err) = full_process(cli) {
        error!("{err:?}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn full_process(cli: Cli) -> Result<(), ScannerError> {
    let bp = blueprint::Data::try_from(
        fs::read_to_string(cli.blueprint).change_context(ScannerError::NoBlueprint)?,
    )
    .change_context(ScannerError::NoBlueprint)?;
    let bp = bp
        .as_blueprint()
        .ok_or(report!(ScannerError::NoBlueprint))?;

    info!("loaded BP");

    let mut mod_list = ModList::generate(&cli.factorio).change_context(ScannerError::SetupError)?;

    // get used mods from preset or detect from BP meta info
    let required_mods = cli.preset.as_ref().map_or(
        bp_helper::get_used_versions(bp).unwrap_or_else(|| {
            std::iter::once(("base".to_owned(), prototypes::targeted_engine_version())).collect()
        }),
        |p| p.used_mods(),
    );

    if !required_mods.is_empty() {
        debug!("checking mod dependencies");

        let used_mods = resolve_mod_dependencies(&required_mods, &mut mod_list)
            .change_context(ScannerError::SetupError)?;

        let missing = mod_list.enable_mods(&used_mods);
        if missing.is_empty() {
            debug!("all mods are already installed");
        } else {
            info!("downloading missing mods from mod portal");
            download_mods(missing, &cli.factorio).change_context(ScannerError::SetupError)?;
        }
    }

    let used_mods = mod_list.active_mods();

    let data_raw = if let Some(path) = cli.prototype_dump {
        DataRaw::load(&path).change_context(ScannerError::SetupError)?
    } else {
        mod_list.save().change_context(ScannerError::SetupError)?;
        debug!("updated mod-list.json");

        // set settings used in BP
        let empty_settings = &HashMap::new();
        let settings =
            bp_helper::get_used_startup_settings(bp).map_or(empty_settings, |settings| settings);
        SettingsDat::load_bp_settings(
            settings,
            bp.info.version,
            &cli.factorio.join("mods/mod-settings.dat"),
        )
        .change_context(ScannerError::SetupError)?
        .save()
        .change_context(ScannerError::SetupError)?;

        debug!("updated mod-settings.dat");

        // execute factorio to dump prototypes
        info!("dumping prototypes");
        std::io::stdout()
            .flush()
            .change_context(ScannerError::SetupError)?;

        let binary_path = cli.factorio.join("bin/x64/run");
        let dump_out = Command::new(cli.factorio_bin.unwrap_or(binary_path))
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
                .attach_printable(String::from_utf8_lossy(&dump_out.stderr).to_string()));
        }

        let dump_path = cli.factorio.join("script-output/data-raw-dump.json");
        DataRaw::load(&dump_path).change_context(ScannerError::SetupError)?
    };

    info!("loaded prototype data");

    // =====[  RENDER TEST  ]=====
    let data = DataUtil::new(data_raw);

    let active_mods = mod_list.active_mods();
    debug!(
        "{} mods active:\n{:?}",
        active_mods.len(),
        active_mods.keys().collect::<Vec<_>>()
    );

    let size = calculate_target_size(bp, &data, cli.target_res, 0.5).unwrap();
    debug!("target size: {size:?}");

    let img = render_bp(
        bp,
        &data,
        &active_mods,
        RenderLayerBuffer::new(size),
        &mut ImageCache::new(),
    );
    info!("render completed");

    let out = fs::File::create(&cli.out).change_context(ScannerError::RenderError)?;
    let enc = png::PngEncoder::new_with_quality(
        out,
        png::CompressionType::Best,
        png::FilterType::default(),
    );

    enc.write_image(img.as_bytes(), img.width(), img.height(), img.color())
        .change_context(ScannerError::RenderError)?;

    info!("saved to {:?}", cli.out);
    Ok(())
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
        let c_box = e_proto.drawing_box();

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
        warn!("unknown entities: {unknown:?}");
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

fn bp_entity2render_opts(value: &blueprint::Entity) -> prototypes::entity::RenderOpts {
    prototypes::entity::RenderOpts {
        position: (&value.position).into(),
        direction: value.direction,
        orientation: value.orientation,
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
    mut render_layers: RenderLayerBuffer,
    image_cache: &mut ImageCache,
) -> image::DynamicImage {
    let rendered_count = bp
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

            if !e.recipe.is_empty() {
                if let Some(icon) = data.get_recipe_icon(
                    &e.recipe,
                    render_layers.scale() * 0.75,
                    used_mods,
                    image_cache,
                ) {
                    debug!(
                        "rendering recipe icon for {} at {:?} [{}]",
                        e.recipe, e.position, e.name
                    );
                    render_layers.add(
                        icon,
                        &render_opts.position,
                        InternalRenderLayer::RecipeOverlay,
                    );
                } else {
                    warn!(
                        "failed to render recipe icon for {} at {:?} [{}]",
                        e.recipe, e.position, e.name
                    );
                }
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

    render_layers.generate_background();
    render_layers.combine()
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
struct PlayerData {
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
struct DependencyResoutionError;

impl Context for DependencyResoutionError {}

impl std::fmt::Display for DependencyResoutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod dependency resolving error")
    }
}

fn resolve_mod_dependencies(
    required: &UsedVersions,
    mod_list: &mut ModList,
) -> Result<UsedVersions, DependencyResoutionError> {
    match mod_list
        .solve_dependencies(required)
        .change_context(DependencyResoutionError)
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

        let Some(info) = factorio_api::blocking::full_info(&name) else {
            warn!("fetching mod info for {name} failed");
            continue;
        };

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
        .change_context(DependencyResoutionError)
}

#[derive(Debug)]
enum ModDownloadError {
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

fn download_mods(missing: UsedVersions, factorio_dir: &Path) -> Result<(), ModDownloadError> {
    let mods_path = factorio_dir.join("mods");

    let (username, token) = {
        let env_username = env::var("FACTORIO_USERNAME").ok();
        let env_token = env::var("FACTORIO_TOKEN").ok();

        if let (Some(username), Some(token)) = (env_username.clone(), env_token.clone()) {
            (username, token)
        } else {
            let player_data = PlayerData::load(&factorio_dir.join("player-data.json"))
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

    for (name, version) in missing {
        ensure!(
            !Mod::wube_mods().contains(&name.as_str()),
            ModDownloadError::TriedToDownloadWubeMod(name, version)
        );

        info!("downloading {name} v{version}");
        let dl = factorio_api::blocking::fetch_mod(&name, &version, &username, &token).ok_or(
            report!(ModDownloadError::DownloadFailed(name.clone(), version)),
        )?;

        fs::write(mods_path.join(format!("{name}_{version}.zip")), dl)
            .change_context(ModDownloadError::SaveFailed(name, version))?;
    }

    Ok(())
}
