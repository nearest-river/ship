

use crate::config::ShipConfig;


pub struct Compiler {
  pub config: ShipConfig
}


impl Compiler {
  pub async fn compile(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn build(&self)-> anyhow::Result<()> {
    unimplemented!()
  }
}





