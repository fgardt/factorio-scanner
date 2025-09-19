use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use mod_util::{mod_info::DependencyVersion, mod_list::ModList};
use scanner::resolve_mod_dependencies;

use factorio_datastage::DataLoader;

#[macro_use]
extern crate log;

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to mods folder
    #[clap(short, long)]
    mod_path: PathBuf,

    /// Path to data folder
    #[clap(short, long)]
    data_path: PathBuf,

    /// Target mod name
    #[clap(short, long)]
    target: String,

    /// Enable the full debug library
    #[clap(long = "fd", action)]
    full_debug: bool,

    /// Dump data
    #[clap(long = "dd", action)]
    dump_data: bool,

    /// Dump history
    #[clap(long = "dh", action)]
    dump_history: bool,
}

fn main() -> ExitCode {
    pretty_env_logger::init_timed();
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let Ok(rt) = tokio::runtime::Runtime::new() else {
        error!("failed to initialize runtime");
        return ExitCode::FAILURE;
    };

    let name = cli.target;
    let mut list = match ModList::generate_custom(cli.data_path, cli.mod_path) {
        Ok(list) => list,
        Err(err) => {
            error!("failed to generate mod list: {err:?}");
            return ExitCode::FAILURE;
        }
    };

    let required = [
        (name.clone(), DependencyVersion::Any),
        ("base".into(), DependencyVersion::Any),
    ]
    .iter()
    .cloned()
    .collect();
    let Ok(used) = rt.block_on(resolve_mod_dependencies(&required, &mut list)) else {
        error!("[{name}] could not resolve dependencies");
        return ExitCode::FAILURE;
    };

    let missing = list.enable_mods(&used);
    if !missing.is_empty() {
        warn!("[{name}] missing deps: {missing:?}");
        return ExitCode::FAILURE;
    }

    let (active, load_order) = list.active_with_order();
    let loader = match DataLoader::init_raw(
        active,
        load_order,
        cli.full_debug,
        cli.dump_data,
        cli.dump_history.then_some(name.clone()),
    ) {
        Ok(loader) => loader,
        Err(err) => {
            error!("[{name}] failed to initialize data loader: {err:?}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = loader.load(".", &name) {
        error!("[{name}] failed to load data: {err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
