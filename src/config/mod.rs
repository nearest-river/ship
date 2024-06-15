
pub mod package;
pub mod metadata;
pub mod dependencies;


use tokio::*;
use package::*;
use dependencies::*;


use io::{
  Error,
  ErrorKind,
};

use std::{
  env,
  ffi::OsStr
};

use crate::{
  consts::path,
  skip_handeling
};

use serde::{
  Deserialize,
  Serialize
};



#[derive(Debug,Serialize,Deserialize)]
pub struct ShipConfig {
  pub package: Package,
  pub dependencies: Dependencies,
}


impl ShipConfig {
  pub async fn save<P: AsRef<std::path::Path>>(&self,path: P)-> io::Result<()> {
    fs::write(path,toml::to_string_pretty(&self).expect("couldn't parse the config file.")).await
  }

  #[must_use]
  pub async fn fetch_config()-> anyhow::Result<ShipConfig> {
    while !env::current_dir()?.eq(OsStr::new("/")) {
      let buf=skip_handeling! {
        fs::read_to_string(path::CONFIG_FILE).await => io::ErrorKind::NotFound => {
          env::set_current_dir("..")?;
          continue
        }
      }?;

      return Ok(toml::from_str::<ShipConfig>(&buf)?);
    }

    Err(Error::new(ErrorKind::PermissionDenied,"cannot read project from root directory.").into())
  }
}

impl Default for ShipConfig {
  fn default()-> Self {
    ShipConfig {
      package: Package::default(),
      dependencies: Default::default()
    }
  }
}


