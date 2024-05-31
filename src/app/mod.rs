
pub mod new;
pub mod init;
pub mod clean;


use new::*;
use init::*;
use clean::*;
use tokio::io;
use clap::Parser;


#[derive(Parser,Debug)]
pub enum App {
  New(New),
  Init(Init),
  Clean(Clean),
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
      Self::Clean(cleaner)=> cleaner.clean().await,
      _=> unimplemented!()
    }
  }
}




