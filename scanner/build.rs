extern crate capnpc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .file("schemas/api.capnp")
        .run()?;

    Ok(())
}
