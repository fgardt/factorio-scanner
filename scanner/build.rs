#[cfg(feature = "server")]
extern crate capnpc;

#[cfg(feature = "server")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .file("schemas/api.capnp")
        .run()?;

    Ok(())
}

fn main() {}
