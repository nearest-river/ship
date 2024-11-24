use tokio::fs;
use semver::VersionReq;
use serde::{
  Serialize,
  Deserialize
};

use std::{
  path::Path,
  collections::HashSet
};


#[derive(Serialize,Deserialize)]
pub struct PackageLock {
  pub packages: HashSet<PackageEntry>
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Default,Debug)]
pub struct PackageEntry {
  pub name: Box<str>,
  pub version: VersionReq,
  pub source: Box<str>,
  pub checksum: Box<str>
}


impl PackageLock {
  pub async fn open<P: AsRef<Path>>(path: P)-> anyhow::Result<Self> {
    let buf=fs::read_to_string(path).await?;
    Ok(toml::from_str::<Self>(&buf)?)
  }

  pub async fn save<P: AsRef<Path>>(&self,path: P)-> anyhow::Result<()> {
    fs::write(path,toml::to_string_pretty(self)?).await?;
    anyhow::Ok(())
  }
}





