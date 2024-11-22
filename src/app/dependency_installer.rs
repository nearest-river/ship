
use clap::Parser;
use semver::VersionReq;

use std::{
  io,
  path::Path,
  str::FromStr
};

use crate::{
  consts::path,
  config::{
    ShipConfig,
    DependencyInfo,
    DependencyTable,
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
  type Err=semver::Error;
  fn from_str(s: &str)-> Result<Self,Self::Err> {
    let mut dependency=s.splitn(2,'@');
    let name=dependency.next().expect("no args were passed").into();
    let version=dependency.next().unwrap_or("latest");

    Ok(Self {
      name,
      version: VersionReq::parse(version)?,
      path: None // TODO
    })
  }
}

impl DependencyInstaller {
  pub async fn add_dependencies(self)-> anyhow::Result<()> {
    let mut config=ShipConfig::open(path::CONFIG_FILE).await?;
    let dependencies=&mut config.dependencies.0;


    for arg in self.args {
      let dep=Dependency::from_str(&arg)?;
      if let Err(err)=dep.fetch().await {
        tracing::error!("{err}");
        continue;
      }

      let dep_info=match &dep.path {
        None=> DependencyInfo::Table(DependencyTable {
          version: Some(dep.version),
          path: None,
          branch: None,
          optional: false,
          registry: None,
          package: None
        }),
        Some(_)=> DependencyInfo::Version(dep.version)
      };

      dependencies.insert(dep.name,dep_info);
    }

    config.save(path::CONFIG_FILE).await?;
    Ok(())
  }
}


impl Dependency {
  pub async fn fetch(&self)-> io::Result<()> {
    Ok(()) // TODO
  }
}






