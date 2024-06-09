
pub mod package;
pub mod metadata;


use tokio::*;
use package::*;

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
}


impl ShipConfig {
  pub async fn save<P: AsRef<std::path::Path>>(&self,path: P)-> io::Result<()> {
    fs::write(path,toml::to_string_pretty(&self).expect("couldn't parse the config file.")).await
  }

  #[must_use]
  pub async fn fetch_config()-> io::Result<ShipConfig> {
    while !env::current_dir()?.eq(OsStr::new("/")) {
      let buf=skip_handeling! {
        fs::read_to_string(path::CONFIG_FILE).await => io::ErrorKind::NotFound => {
          env::set_current_dir("..")?;
          continue
        }
      }?;

      return Ok(toml::from_str::<ShipConfig>(&buf).unwrap());
    }

    Err(Error::new(ErrorKind::PermissionDenied,"cannot read project from root directory."))
  }
}

impl Default for ShipConfig {
  fn default()-> Self {
    ShipConfig {
      package: Package::default()
    }
  }
}


