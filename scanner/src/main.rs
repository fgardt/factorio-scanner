#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use error_stack::{Context, Result, ResultExt};

#[allow(clippy::wildcard_imports)]
use scanner::*;

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets the used logging level
    /// Possible values: error, warn, info, debug, trace
    /// For no logging don't set this option
    /// Note: the LOG_LEVEL environment variable overrides this option
    #[clap(long, value_parser, verbatim_doc_comment)]
    log_level: Option<log::Level>,

    /// Path to the factorio directory that contains the data folder (path.read-data)
    #[clap(short, long, value_parser)]
    factorio: PathBuf,

    /// Path to the factorio binary instead of the default expected one
    #[clap(long, value_parser)]
    factorio_bin: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render a blueprint string
    Render {
        #[clap(subcommand)]
        input: Input,

        /// Path to the data dump json file. If not set, the data will be dumped automatically
        #[clap(long, value_parser)]
        prototype_dump: Option<PathBuf>,

        /// Preset to use
        #[clap(long, value_enum)]
        preset: Option<preset::Preset>,

        /// List of additional mods to use
        #[clap(long, value_parser, use_value_delimiter = true, value_delimiter = ',')]
        mods: Vec<String>,

        /// Path to the output file
        #[clap(short, long, value_parser)]
        out: PathBuf,

        /// Target resolution (1 side of a square) in pixels
        #[clap(long = "res", default_value_t = 2048.0)]
        target_res: f64,

        /// Minimum scale to use (below 0.5 makes not much sense, vanilla HR mode is 0.5)
        #[clap(long, default_value_t = 0.5)]
        min_scale: f64,
    },
}

#[derive(Subcommand, Debug)]
enum Input {
    String {
        /// The blueprint string
        #[clap(value_parser)]
        string: String,
    },

    File {
        /// Path to the file that contains your blueprint string
        #[clap(value_parser)]
        file: PathBuf,
    },
}

#[derive(Debug)]
struct BlueprintInputError;

impl Context for BlueprintInputError {}

impl std::fmt::Display for BlueprintInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "blueprint input error")
    }
}

impl Input {
    fn get_bp_string(self) -> Result<String, BlueprintInputError> {
        match self {
            Self::String { string } => Ok(string),
            Self::File { file } => fs::read_to_string(file).change_context(BlueprintInputError),
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

    info!(
        "starting {} v{} with prototypes v{} & types v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        prototypes::targeted_engine_version(),
        types::targeted_engine_version()
    );

    let factorio_bin = cli
        .factorio_bin
        .unwrap_or_else(|| cli.factorio.join("bin/x64/factorio"));

    if let Err(err) = match cli.command {
        Commands::Render {
            input,
            prototype_dump,
            preset,
            mods,
            out,
            target_res,
            min_scale,
        } => render_command(
            input,
            &cli.factorio,
            &factorio_bin,
            preset,
            &mods,
            prototype_dump,
            target_res,
            &out,
        ),
    } {
        error!("{err:#?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[allow(clippy::too_many_arguments)]
fn render_command(
    input: Input,
    factorio: &Path,
    factorio_bin: &Path,
    preset: Option<preset::Preset>,
    mods: &[String],
    prototype_dump: Option<PathBuf>,
    target_res: f64,
    out: &Path,
) -> Result<(), ScannerError> {
    let bp_string = input
        .get_bp_string()
        .change_context(ScannerError::NoBlueprint)?;

    let bp = blueprint::Data::try_from(bp_string).change_context(ScannerError::NoBlueprint)?;
    let (data, active_mods) = load_data(&bp, factorio, factorio_bin, preset, mods, prototype_dump)?;
    let (res, missing, thumb) = render(&bp, &data, &active_mods, target_res)?;

    if !missing.is_empty() {
        warn!("missing prototypes: {missing:?}");
    }

    fs::write(out, res).change_context(ScannerError::RenderError)?;
    info!("saved render to {out:?}");

    if let Some(thumb) = thumb {
        fs::write(out.with_extension("thumb.png"), thumb)
            .change_context(ScannerError::RenderError)?;
        info!("saved thumbnail to {:?}", out.with_extension("thumb.png"));
    }

    Ok(())
}
