#![allow(clippy::expect_used)]

use std::path::PathBuf;

use clap::Parser;
use mod_util::UsedMods;
use prototypes::DataUtil;
use scanner::preset::Preset;

#[derive(Parser)]
struct Cli {
    /// Path to the factorio application folder (should contain 'data' and 'mods' folders)
    #[clap(short, long, value_parser)]
    factorio: PathBuf,

    /// Path to the factorio binary to use
    #[clap(short = 'b', long, value_parser)]
    factorio_bin: Option<PathBuf>,

    /// Path to the folder containing blueprints to render
    bp_folder: PathBuf,

    /// Path to the output folder for rendered images
    output_folder: PathBuf,

    /// Select a specific blueprint to render (optional)
    #[clap(short, long)]
    select: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    let factorio_bin = cli
        .factorio_bin
        .unwrap_or_else(|| cli.factorio.join("bin/x64/factorio"));

    let dummy_bp = blueprint::Data::Blueprint(blueprint::Blueprint::default());
    let (raw_data, used_mods) = rt
        .block_on(scanner::load_data(
            &dummy_bp,
            &cli.factorio,
            &cli.factorio,
            &factorio_bin,
            Some(Preset::SpaceAge),
            &[],
            None,
        ))
        .expect("Failed to load data");

    render_folder(
        &cli.bp_folder,
        &cli.output_folder,
        &raw_data,
        &used_mods,
        cli.select.as_ref(),
    );
}

fn render_folder(
    folder: &PathBuf,
    output_folder: &PathBuf,
    raw_data: &DataUtil,
    used_mods: &UsedMods,
    filter: Option<&String>,
) {
    for entry in std::fs::read_dir(folder).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            let sub_output_folder =
                output_folder.join(path.file_name().expect("Failed to get folder name"));
            render_folder(&path, &sub_output_folder, raw_data, used_mods, filter);
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }

        if let Some(filter) = filter {
            if path
                .file_stem()
                .is_none_or(|name| name.to_string_lossy() != filter.as_str())
            {
                continue;
            }
        }

        println!("Rendering blueprint: {}", path.display());

        let bp_data = std::fs::read_to_string(&path)
            .expect("Failed to read blueprint file")
            .try_into()
            .expect("Failed to parse blueprint data");

        std::fs::create_dir_all(output_folder).expect("Failed to create output folder");
        let output_path = output_folder.join(path.file_stem().expect("Failed to get file name"));

        let (res, _, thumb) = scanner::render(&bp_data, raw_data, used_mods, 2048.0, 0.5)
            .expect("Failed to render blueprint");

        std::fs::write(output_path.with_extension("png"), res)
            .expect("Failed to write rendered image");

        if let Some(thumb) = thumb {
            std::fs::write(output_path.with_extension("thumb.png"), thumb)
                .expect("Failed to write thumbnail image");
        } else {
            std::fs::remove_file(output_path.with_extension("thumb.png"))
                .expect("Failed to remove thumbnail image");
        }
    }
}
