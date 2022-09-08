/// This file is used to build the Rust protobuf libraries used by the example
/// project.
use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    // The toplevel types directory.

    let proto_files = &[
        concat!(env!("CARGO_MANIFEST_DIR"), "/../protobufs/messages.proto"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../protobufs/service.proto"),
    ];
    let include_dirs = &[concat!(env!("CARGO_MANIFEST_DIR"), "/../protobufs/")];

    tonic_build::configure()
        // Ensure that all protobufs have Serde traits defined, so we can JSON-serialize them
        // when needed, e.g., for logging.
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .build_server(true)
        .compile(proto_files, include_dirs)
        .context("Unable to compile protobufs")?;

    Ok(())
}
