
pub mod new;
pub mod init;

use new::*;
use init::*;
use tokio::io;
use clap::Parser;


#[derive(Parser,Debug)]
pub enum App {
  New(New),
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
      Self::Init(this)=> this.run().await,
      Self::New(this)=> this.run().await,
      _=> unimplemented!()
    }
  }
}




