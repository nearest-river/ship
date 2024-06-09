
use super::Init;
use clap::Parser;
use std::path::Path;
use crate::consts::event;

use tokio::{
  fs,
  io
};


#[derive(Parser,Debug)]
pub struct New {
  path: Box<Path>,
  #[arg(long,default_value="git")]
  vcs: Box<str>,
  #[arg(long)]
  bin: bool,
  #[arg(long)]
  lib: bool,
  #[arg(long)]
  edition: Option<u16>,
  #[arg(long,default_value="")]
  name: Box<str>,
  #[arg(long,default_value="todo")]
  registry: Box<str>,
  #[arg(long,default_value="false")]
  quite: bool,
  #[arg(long,default_value="")]
  config: Box<str>,
  #[arg(short='Z',default_value="")]
  flags: Box<str>
}

impl New {
  pub async fn run(self)-> anyhow::Result<()> {
    if let Err(err)=fs::create_dir(&self.path).await {
      match err.kind() {
        io::ErrorKind::AlreadyExists=> tracing::error!("{}: destination `{}` already exists.\n\nUse `ship init` to initialize the directory",event::ERROR,self.path.display()),
        _=> tracing::error!("{err}")
      }
      std::process::exit(err.raw_os_error().unwrap_or(1));
    }

    let New { path,vcs,bin,lib,edition,name,registry,quite,config,flags }=self;
    Init { path,vcs,bin,lib,edition,name,registry,quite,config,flags }.run().await
  }
}

