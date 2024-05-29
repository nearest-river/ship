

use tokio::*;

use serde::{
  Deserialize,
  Serialize
};



#[derive(Debug,Serialize,Deserialize)]
pub struct ShipConfig {
}


impl ShipConfig {
  pub async fn save<P: AsRef<std::path::Path>>(&self,_path: P)-> io::Result<()> {
    unimplemented!()
  }
}

impl Default for ShipConfig {
  fn default()-> Self {
    unimplemented!()
  }
}


