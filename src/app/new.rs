
use super::Init;
use clap::Parser;
use std::path::Path;
use color_print::cstr;

use tokio::{
  fs,
  io
};


#[derive(Parser,Debug)]
pub struct New {
  path: Box<Path>,
  #[arg(long)]
  pub bin: bool,
  #[arg(long)]
  pub lib: bool,
  #[arg(long)]
  pub edition: Option<u16>,
  #[arg(long,default_value="")]
  pub name: Box<str>,
  #[arg(long,default_value="todo")]
  pub registry: Box<str>,
  #[arg(long,default_value="false")]
  pub quite: bool,
  #[arg(long,default_value="")]
  pub config: Box<str>,
  #[arg(short='Z',default_value="")]
  pub flags: Box<str>
}

impl New {
  pub async fn run(self)-> anyhow::Result<()> {
    if let Err(err)=fs::create_dir(&self.path).await {
      match err.kind() {
        io::ErrorKind::AlreadyExists=> panic!("{}: destination `{}` already exists.\n\nUse `ship init` to initialize the directory",cstr!("<#ff0000>error</#ff0000>"),self.path.display()),
        _=> panic!("{err}")
      }
    }

    let New { path,bin,lib,edition,name,registry,quite,config,flags }=self;
    Init { path,bin,lib,edition,name,registry,quite,config,flags }.run().await
  }
}

