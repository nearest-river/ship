use semver::VersionReq;
use super::PackageLock;

use std::{
  path::Path,
  collections::HashMap
};

use serde::{
  Serialize,
  Deserialize
};




#[derive(Debug,Serialize,Deserialize,Default)]
pub struct Dependencies(pub HashMap<Box<str>,DependencyInfo>);


#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum DependencyInfo {
  Version(VersionReq),
  Table(DependencyTable)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct DependencyTable {
  pub version: Option<VersionReq>,
  #[serde(rename="git",alias="hg",alias="pijul",alias="fossil",alias="path")]
  pub path: Option<Box<str>>,
  pub branch: Option<Box<str>>,
  #[serde(default="_false")]
  pub optional: bool,
  pub registry: Option<Box<str>>,
  pub package: Option<Box<str>>,
}

const fn _false()-> bool { false }

impl Dependencies {
  pub async fn fetch(&self)-> anyhow::Result<PackageLock> {
    unimplemented!()
  }

  pub async fn download(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn sync<P: AsRef<Path>>(&self,_path: P)-> anyhow::Result<PackageLock> {
    unimplemented!()
  }
}


