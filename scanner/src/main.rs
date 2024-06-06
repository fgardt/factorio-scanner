#![allow(dead_code, clippy::upper_case_acronyms, unused_variables)]

use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use error_stack::{Context, Result, ResultExt};
use tracing::{error, info, warn};

#[allow(clippy::wildcard_imports)]
use scanner::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the factorio application directory, which contains the 'data' folder (path.read-data)
    #[clap(short, long, value_parser)]
    factorio: Option<PathBuf>,

    /// Path to the factorio user data directory (path.write-data), which contains the 'mods' and 'script-output' folders
    #[clap(long, value_parser)]
    factorio_userdir: Option<PathBuf>,

    /// Path to the factorio binary instead of the default expected one
    #[clap(long, value_parser)]
    factorio_bin: Option<PathBuf>,

    #[clap(flatten)]
    args: CommandArgs,
}

#[derive(Parser, Debug)]
struct CommandArgs {
    /// Blueprint string or file to render
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
}

#[derive(Subcommand, Debug)]
enum Input {
    /// Provide a blueprint string directly
    String {
        /// The blueprint string
        #[clap(value_parser)]
        string: String,
    },

    /// Path to a file that contains a blueprint string
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
    pretty_env_logger::init();

    info!(
        "starting {} v{} with prototypes v{} & types v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        prototypes::targeted_engine_version(),
        types::targeted_engine_version()
    );

    let (factorio_appdir, factorio_userdir, factorio_bin) = match infer_paths(&cli) {
        Ok(tup) => tup,
        Err(err) => {
            error!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .change_context(ScannerError::ServerError)
    {
        Ok(rt) => rt,
        Err(err) => {
            error!("{err:#?}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = rt.block_on(render_command(
        cli.args.input,
        &factorio_appdir,
        &factorio_userdir,
        &factorio_bin,
        cli.args.preset,
        &cli.args.mods,
        cli.args.prototype_dump,
        cli.args.target_res,
        cli.args.min_scale,
        &cli.args.out,
    )) {
        error!("{err:#?}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn get_home(argument: &str) -> std::result::Result<PathBuf, String> {
    match env::var("HOME") {
        Ok(home) => Ok(home.into()),
        Err(e) => Err(format!("Couldn't infer {argument} ($HOME: {e})")),
    }
}

fn infer_paths(cli: &Cli) -> std::result::Result<(PathBuf, PathBuf, PathBuf), String> {
    let factorio_appdir = cli.factorio.clone().map_or_else(
        || match env::consts::OS {
            "linux" => Ok(Path::new(&get_home("--factorio")?).join(".factorio")),
            "macos" => Ok(Path::new("/Applications/factorio.app/Contents").to_path_buf()),
            default => Err("--factorio is required".to_owned()),
        },
        Ok,
    )?;

    if !factorio_appdir.join("data").is_dir() {
        return Err(format!(
            "Factorio app directory at {factorio_appdir:?} doesn't exist \
            or doesn't contain 'data', check --factorio"
        ));
    }

    let factorio_userdir = cli.factorio_userdir.clone().map_or_else(
        || match env::consts::OS {
            "macos" => Ok(Path::new(&get_home("--factorio-userdir")?)
                .join("Library/Application Support/factorio")),
            default => Ok(factorio_appdir.clone()),
        },
        Ok::<PathBuf, String>,
    )?;

    if !factorio_userdir.join("mods").is_dir() {
        return Err(format!(
            "Factorio user data directory at {factorio_userdir:?} doesn't exist \
            or doesn't contain 'mods', check --factorio-userdir"
        ));
    }

    let factorio_bin = cli
        .factorio_bin
        .clone()
        .unwrap_or_else(|| match env::consts::OS {
            "macos" => factorio_appdir.join("MacOS/factorio"),
            default => factorio_appdir.join("bin/x64/factorio"),
        });

    if !factorio_bin.exists() {
        return Err(format!(
            "Factorio binary not found at {factorio_bin:?}, check --factorio-bin"
        ));
    }

    Ok((factorio_appdir, factorio_userdir, factorio_bin))
}

#[allow(clippy::too_many_arguments)]
async fn render_command(
    input: Input,
    factorio: &Path,
    factorio_userdir: &Path,
    factorio_bin: &Path,
    preset: Option<preset::Preset>,
    mods: &[String],
    prototype_dump: Option<PathBuf>,
    target_res: f64,
    min_scale: f64,
    out: &Path,
) -> Result<(), ScannerError> {
    let bp_string = input
        .get_bp_string()
        .change_context(ScannerError::NoBlueprint)?;

    let bp = blueprint::Data::try_from(bp_string).change_context(ScannerError::NoBlueprint)?;
    let (data, active_mods) = load_data(
        &bp,
        factorio,
        factorio_userdir,
        factorio_bin,
        preset,
        mods,
        prototype_dump,
    )
    .await?;
    let (res, missing, thumb) = render(&bp, &data, &active_mods, target_res, min_scale)?;

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
