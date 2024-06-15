
use semver::VersionReq;

use serde::{
  Serialize,
  Deserialize
};

use std::{
  path::Path,
  collections::HashMap
};



#[derive(Debug,Serialize,Deserialize,Default)]
pub struct Dependencies(HashMap<Box<str>,DependencyInfo>);


#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum DependencyInfo {
  Version(VersionReq),
  Table(DependencyTable)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct DependencyTable {
  version: Option<VersionReq>,
  path: Option<Box<Path>>,
  #[serde(rename="git",alias="hg",alias="pujul",alias="fossil")]
  vcs_url: Option<Box<str>>,
  branch: Option<Box<str>>,
  #[serde(default="_false")]
  optional: bool,
  registry: Option<Box<str>>,
  package: Option<Box<str>>,
}

const fn _false()-> bool { false }

impl Dependencies {
  pub async fn fetch(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn download(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn build()-> anyhow::Result<()> {
    unimplemented!()
  }
}


