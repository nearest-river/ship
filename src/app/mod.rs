

pub mod init;

use init::*;
use tokio::io;
use clap::Parser;


#[derive(Parser,Debug)]
pub enum App {
  New,
  Init(Init),
  Clean,
  Check,
  Build,
  Run,
  Test,
  Bench,
  Add,
  Remove,
  Install,
  Uninstall,
  Publish,
  Doc
}


impl App {
  pub async fn run(self)-> io::Result<()> {
    match self {
      Self::Init(mut this)=> this.run().await,
      _=> unimplemented!()
    }
  }
}




