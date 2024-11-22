
pub mod package;
pub mod metadata;
pub mod dependencies;


use tokio::*;
pub use package::*;
use std::path::Path;
pub use dependencies::*;

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
  pub async fn save<P: AsRef<Path>>(&self,path: P)-> io::Result<()> {
    fs::write(path,toml::to_string_pretty(&self).expect("couldn't parse the config file.")).await
  }

  #[must_use]
  pub async fn open<P: AsRef<Path>>(path: P)-> anyhow::Result<ShipConfig> {
    let s=fs::read_to_string(path).await?;
    Ok(toml::from_str::<Self>(&s)?)
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


