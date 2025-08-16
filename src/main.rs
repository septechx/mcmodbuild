mod binary;
mod cli;
mod installer;
mod macros;
mod structs;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::{anyhow, Result};
use binary::deserialize;
use clap::Parser;
use colored::Colorize;
use installer::Installer;
use oxfmt::Serializable;
use structs::ModBuild;

use crate::cli::{McModBuild, Subcommands};

fn main() {
    if let Err(err) = __main() {
        eprintln!("{}", err.to_string().red().bold());
        process::exit(1);
    }
}

fn __main() -> Result<()> {
    let cli = McModBuild::parse();

    match cli.subcommand {
        Subcommands::Build { file, destination } => build(file, destination),
        Subcommands::Install { file, destination } => install(file, destination),
        Subcommands::Init { destination } => {
            init(destination).map_err(|err| anyhow!("Failed to create files: {err}"))
        }
    }
}

fn init(destination: Option<PathBuf>) -> Result<()> {
    let destination = destination.unwrap_or(env::current_dir()?);

    fs::write(
        destination.join("mymod.yml"),
        serde_yml::to_string(&ModBuild::default())?,
    )?;

    println!(
        "{}{}",
        "Successfully created build file at ".green(),
        destination.to_string_lossy().bright_green().bold()
    );

    Ok(())
}

fn build(file: PathBuf, destination: Option<PathBuf>) -> Result<()> {
    let content = fs::read_to_string(file)?;
    let build: ModBuild = serde_yml::from_str(&content)?;

    let id = build.id.clone();
    let name = format!("{id}.mcmodbuild");
    let path: PathBuf = destination.unwrap_or(PathBuf::from(&name));

    fs::write(path, build.serialize()?)?;

    Ok(())
}

fn install(file: PathBuf, destination: Option<PathBuf>) -> Result<()> {
    let content = fs::read_to_string(file)?;
    let build = deserialize(content.as_bytes())?;

    let id = build.id.clone();
    let branch = build.branch.clone();
    let name = format!("{id}-{branch}.jar");
    let destination = destination.unwrap_or(Path::new(&name).to_path_buf());

    let installer = Installer::new(build)?;
    installer.install(destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::structs::{BuildType, ExcludePair, ExcludeType};

    use super::*;

    #[test]
    fn roundtrip_serialize_deserialize() {
        let build = ModBuild {
            id: "testmod".into(),
            name: "Test mod".into(),
            git: "https://repo.git".into(),
            branch: "1.21.7".into(),
            build: BuildType::Cmd,
            cmd: Some("./gradlew build".into()),
            out: "@/target/".into(),
            exclude: vec![
                ExcludePair {
                    type_name: ExcludeType::Ends,
                    value: "-source.jar".into(),
                },
                ExcludePair {
                    type_name: ExcludeType::Starts,
                    value: "dev-".into(),
                },
            ],
        };

        let serialized = build.clone().serialize().unwrap();
        println!("Serialized: {serialized:?}");
        let deserialized = deserialize(&serialized).unwrap();
        println!("Deserialized: {deserialized:?}");
        assert_eq!(build, deserialized);
    }
}
