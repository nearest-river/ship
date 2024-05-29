
use url::Url;
use std::path::Path;
use semver::Version;
use super::metadata::Metadata;

use serde::{
  Serialize,
  Deserialize
};



#[derive(Debug,Serialize,Deserialize)]
pub struct Package {
  pub name: String,
  pub edition: u16,
  pub version: Option<Version>,
  pub c_version: Option<Version>,
  pub authors: Option<Vec<Box<str>>>,
  pub description: Option<String>,
  pub documentation: Option<Box<Path>>,
  pub readme: Option<Box<Path>>,
  pub homepage: Option<Url>,
  pub repository: Option<Url>,
  pub license: Option<Box<str>>,
  pub license_file: Option<Box<Path>>,
  pub keywords: Option<Vec<Box<str>>>,
  pub categories: Option<Vec<Box<str>>>,
  pub build: Option<Box<Path>>,
  pub links: Option<Box<str>>,
  pub exclude: Option<Vec<Box<str>>>,
  pub include: Option<Vec<Box<str>>>,
  pub publish: Option<Publish>,
  pub metadata: Option<Metadata>,
  pub default_run: Option<Box<str>>,
  pub autobins: Option<bool>,
  pub autoexamples: Option<bool>,
  pub autotests: Option<bool>,
  pub autobenches: Option<bool>,
  pub resolver: Option<u8>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum Publish {
  Bool(bool),
  Name(Box<str>)
}


impl Default for Package {
  fn default()-> Self {
    Package {
      name: String::new(),
      edition: 2020,
      version: None,
      c_version: None,
      authors: None,
      description: None,
      documentation: None,
      readme: None,
      homepage: None,
      repository: None,
      license: None,
      license_file: None,
      keywords: None,
      categories: None,
      build: None,
      links: None,
      exclude: None,
      include: None,
      publish: None,
      metadata: None,
      default_run: None,
      autobins: None,
      autoexamples: None,
      autotests: None,
      autobenches: None,
      resolver: None,
    }
  }
}

