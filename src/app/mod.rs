
pub mod new;
pub mod init;
pub mod clean;
pub mod build;


use new::*;
use init::*;
use clean::*;
use build::*;
use clap::Parser;


#[derive(Parser,Debug)]
pub enum App {
  New(New),
  Init(Init),
  Clean(Clean),
  Check,
  Build(Build),
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
  pub async fn run(self)-> anyhow::Result<()> {
    match self {
      Self::Init(this)=> this.run().await,
      Self::New(this)=> this.run().await,
      Self::Clean(cleaner)=> cleaner.clean().await,
      Self::Build(builder)=> builder.build().await,
      _=> unimplemented!()
    }
  }
}




