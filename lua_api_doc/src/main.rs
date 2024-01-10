use clap::Parser;
use diff::Diff;

pub mod format;
use format::DiffPrint;

use crate::format::prototype::PrototypeDoc;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Base version of the docs to use
    #[clap(short, long, value_parser)]
    source: String,

    /// Target version of the docs to compare against
    /// If not specified, the latest version is used
    #[clap(short, long, value_parser)]
    target: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let source = reqwest::blocking::get(format!(
        "https://lua-api.factorio.com/{}/prototype-api.json",
        cli.source
    ))
    .unwrap()
    .bytes()
    .unwrap();

    let target = reqwest::blocking::get(format!(
        "https://lua-api.factorio.com/{}/prototype-api.json",
        cli.target.unwrap_or("latest".to_owned())
    ))
    .unwrap()
    .bytes()
    .unwrap();

    let source: PrototypeDoc = serde_json::from_slice(&source).unwrap();
    let target: PrototypeDoc = serde_json::from_slice(&target).unwrap();

    proto_info(&source);
    println!();
    proto_info(&target);

    // calculate the diff
    let diff = source.diff(&target);

    println!("\n=> {} prototypes changed", diff.prototypes.0.len());
    println!("=> {} types changed\n", diff.types.0.len());

    // print the diff
    diff.diff_print(&source, &target, 0, "");

    // let d = serde_json::to_string_pretty(&diff).unwrap();
    // println!("{d}");
}

fn proto_info(proto: &format::prototype::PrototypeDoc) {
    println!(
        "{:?} @ {}: {:?}",
        proto.common.application, proto.common.application_version, proto.common.stage
    );
    println!("  {} prototypes", proto.prototypes.len());
    println!("  {} types", proto.types.len());
}
