
pub mod new;
pub mod init;
pub mod clean;
pub mod build;
pub mod dependency_installer;


use new::*;
use init::*;
use clean::*;
use build::*;
use clap::Parser;
use dependency_installer::*;


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
  Add(DependencyInstaller),
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
      Self::Add(dependency_installer)=> dependency_installer.add_dependencies().await,
      _=> unimplemented!()
    }
  }
}




