use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let protos_dir = "proto";

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("guild_descriptor.bin"))
        .compile_protos(&["guild.proto"], &[protos_dir])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("logs_descriptor.bin"))
        .compile_protos(&["logs.proto"], &[protos_dir])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("moderation_descriptor.bin"))
        .compile_protos(&["moderation.proto"], &[protos_dir])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tickets_descriptor.bin"))
        .compile_protos(&["tickets.proto"], &[protos_dir])?;

    Ok(())
}
