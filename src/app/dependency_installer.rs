
use clap::Parser;
use semver::VersionReq;
use std::{
  path::Path,
  str::FromStr,
  io::{
    self,
    Error
  }
};


#[derive(Parser,Debug)]
pub struct DependencyInstaller {
  args: Vec<Box<str>>
}

#[derive(Debug,Clone)]
pub struct Dependency {
  name: Box<str>,
  version: VersionReq,
  path: Option<Box<Path>> //TODO
}

impl FromStr for Dependency {
  type Err=Error;
  fn from_str(s: &str)-> Result<Self,Self::Err> {
    todo!("{s}")
  }
}

impl DependencyInstaller {
  pub async fn add_dependencies(self)-> anyhow::Result<()> {
    for arg in self.args {
      
    }

    Ok(())
  }
}


impl Dependency {
  fn fetch(&self)-> io::Result<()> {
    Ok(())
  }
}

