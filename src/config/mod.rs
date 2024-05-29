
pub mod package;
pub mod metadata;

use package::*;
use tokio::*;

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
}

impl Default for ShipConfig {
  fn default()-> Self {
    ShipConfig {
      package: Package::default()
    }
  }
}


