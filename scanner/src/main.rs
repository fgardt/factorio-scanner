#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
};

use clap::Parser;
use image::GenericImageView;

use mod_util::mod_settings::SettingsDat;
use prototypes::{EntityRenderOpts, EntityType, RenderableEntity};
use types::{merge_renders, ConnectedDirections, Direction, GraphicsOutput, ImageCache};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file that contains your blueprint string
    #[clap(short, long, value_parser)]
    blueprint: PathBuf,

    /// Path to the factorio directory that contains the data folder (path.read-data)
    #[clap(short, long, value_parser)]
    factorio: PathBuf,

    /// Path to the data dump json file
    #[clap(short, long, value_parser)]
    dump: PathBuf,

    /// Path to the output file
    #[clap(short, long, value_parser)]
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let bp = blueprint::Data::try_from(fs::read_to_string(cli.blueprint).unwrap()).unwrap();
    let bp = bp.as_blueprint().unwrap();

    println!("loaded BP");

    // detect mods from meta info (only the very fist BP is checked)
    let used_mods = bp.get_used_mods().map_or_else(HashMap::new, |mods| mods);
    let used_settings = bp
        .get_used_startup_settings()
        .map_or_else(HashMap::new, std::clone::Clone::clone);

    if used_mods.is_empty() {
        println!("no meta info found, assuming vanilla BP");
    } else {
        println!("used mods: {used_mods:?}");
    }

    let settings_path = cli.factorio.join("mods/mod-settings.dat");
    let old_settings = SettingsDat::load(&settings_path).unwrap();

    // set settings used in BP
    SettingsDat::load_bp_settings(bp, &settings_path)
        .unwrap()
        .save()
        .unwrap();

    let data_raw = prototypes::DataRaw::load(&cli.dump).unwrap();

    println!("loaded prototype data");

    // // =====[  RENDER TEST  ]=====
    let data = prototypes::DataUtil::new(data_raw);

    match render_bp(bp, &data, &cli.factorio, &used_mods, &mut ImageCache::new()) {
        Some((img, scale, (shift_x, shift_y))) => {
            println!("render done");

            let img = img.resize(
                img.dimensions().0 / 4,
                img.dimensions().1 / 4,
                image::imageops::FilterType::CatmullRom,
            );
            println!(
                "BP: {}x{} x{scale} ({shift_x}, {shift_y})",
                img.dimensions().0,
                img.dimensions().1,
            );

            img.save(cli.out).unwrap();
        }
        None => println!("EMPTY BP!"),
    }

    // restore previous settings
    old_settings.save().unwrap();
}

fn render_entity(
    name: &str,
    entity: &dyn RenderableEntity,
    render_opts: &EntityRenderOpts,
    image_cache: &mut ImageCache,
) {
    match entity.render(render_opts, image_cache) {
        Some((img, scale, (shift_x, shift_y))) => {
            // println!(
            //     "{name}: {}x{} x{scale} ({shift_x}, {shift_y})",
            //     img.dimensions().0,
            //     img.dimensions().1,
            // );

            img.save(format!("render_test/{name}.png")).unwrap();
        }
        None => {
            println!("{name}: NO SPRITE!");
        }
    }
}

fn render_by_name(
    name: &str,
    data: &prototypes::DataUtil,
    render_opts: &EntityRenderOpts,
    image_cache: &mut ImageCache,
) {
    match data.render_entity(name, render_opts, image_cache) {
        Some((img, scale, (shift_x, shift_y))) => {
            println!(
                "{name}: {}x{} x{scale} ({shift_x}, {shift_y})",
                img.dimensions().0,
                img.dimensions().1,
            );

            img.save(format!("render_test/{name}.png")).unwrap();
        }
        None => {
            println!("{name}: NO SPRITE!");
        }
    }
}

fn bp_entity2render_opts<'a>(
    value: &blueprint::Entity,
    factorio_dir: &'a Path,
    used_mods: HashMap<&'a str, &'a str>,
) -> prototypes::EntityRenderOpts<'a> {
    prototypes::EntityRenderOpts {
        factorio_dir,
        used_mods,
        direction: value.direction,
        orientation: value.orientation.map(f64::from),
        pickup_position: value
            .pickup_position
            .as_ref()
            .map(|v| (f64::from(v.x), f64::from(v.y))),
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
    factorio_dir: &Path,
    used_mods: &HashMap<&str, &str>,
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

            let mut render_opts = bp_entity2render_opts(e, factorio_dir, used_mods.clone());
            render_opts.connections = connections;
            render_opts.connected_gates = connected_gates;
            render_opts.draw_gate_patch = draw_gate_patch;

            data.render_entity(&e.name, &render_opts, image_cache).map(
                |(img, scale, (shift_x, shift_y))| {
                    Some((
                        img,
                        scale,
                        (
                            shift_x + f64::from(e.position.x),
                            shift_y + f64::from(e.position.y),
                        ),
                    ))
                },
            )
        })
        .collect::<Vec<_>>();

    println!("entities: {}, layers: {}", bp.entities.len(), renders.len());

    merge_renders(renders.as_slice())
}
