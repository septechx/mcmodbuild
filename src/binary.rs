use crate::structs::{BuildType, ModBuild};
use anyhow::{Ok, Result};
use oxfmt::Deserialize;

pub fn deserialize(buf: &[u8]) -> Result<ModBuild> {
    let header = "mcmodbuild".as_bytes();
    let version: u16 = 1;

    let mut deserialize = Deserialize::new(buf, header, version)?;
    let id = deserialize.read()?;
    let name = deserialize.read()?;
    let git = deserialize.read()?;
    let branch = deserialize.read()?;
    let build: BuildType = deserialize.read::<u8>()?.into();
    let cmd = match build {
        BuildType::Cmd => Some(deserialize.read()?),
        BuildType::Std => None,
    };
    let out = deserialize.read()?;
    let exclude = deserialize.read()?;

    Ok(ModBuild {
        id,
        name,
        git,
        branch,
        build,
        cmd,
        out,
        exclude,
    })
}
