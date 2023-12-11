use diff::Diff;

pub mod format;

fn main() {
    let proto94 = format::prototype::PrototypeDoc::load("proto.1.1.94.json").unwrap();
    let proto100 = format::prototype::PrototypeDoc::load("proto.1.1.100.json").unwrap();

    proto_info(&proto94);
    proto_info(&proto100);

    // calculate the diff
    let diff = proto94.diff(&proto100);

    println!("{} prototypes changed", diff.prototypes.0.len());
    println!("{} types changed", diff.types.0.len());

    print!("Prototypes changed: ");
    for p in diff.prototypes.0 {
        match p {
            diff::VecDiffType::Removed { index, .. } => {
                print!("-{}, ", proto94.prototypes[index].common.name)
            }
            diff::VecDiffType::Altered { index, .. } => {
                print!("*{}, ", proto94.prototypes[index].common.name)
            }
            diff::VecDiffType::Inserted { index, .. } => {
                print!("+{}, ", proto100.prototypes[index].common.name)
            }
        }
    }
    println!();
}

fn proto_info(proto: &format::prototype::PrototypeDoc) {
    println!(
        "{:?} @ {}: {:?}",
        proto.common.application, proto.common.application_version, proto.common.stage
    );
    println!("{} prototypes", proto.prototypes.len());
    println!("{} types", proto.types.len());
}
