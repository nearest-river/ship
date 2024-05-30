
use color_print::cstr;
use tokio::*;
use clap::Parser;
use std::path::Path;
use crate::fs_api;

use super::Init;



#[derive(Parser,Debug)]
pub struct New {
  path: Box<Path>,
  #[arg(long,default_value="true")]
  pub bin: bool,
  #[arg(long,default_value="false")]
  pub lib: bool,
  #[arg(long,default_value="2015")]// TODO
  pub edition: u16,
  #[arg(long,default_value="todo")]
  pub name: Box<str>,
  #[arg(long,default_value="todo")]
  pub registry: Box<str>,
  #[arg(long,default_value="false")]
  pub quite: bool,
  #[arg(long,default_value="")]
  pub config: Box<str>,
  #[arg(long,default_value="")]
  pub flags: Box<str>
}

impl New {
  pub async fn run(self)-> io::Result<()> {
    if let Err(err)=fs::create_dir(&self.path).await {
      match err.kind() {
        io::ErrorKind::AlreadyExists=> panic!("{}: destination `{}` already exists.\n\nUse `ship init` to initialize the directory",cstr!("<#ff0000>error</#ff0000>"),self.path.display()),
        _=> panic!("{err}")
      }
    }
    fs_api::ensure_fresh_dir(&self.path).await?;

    let New { path,bin,lib,edition,name,registry,quite,config,flags }=self;
    Init {
      path: Some(path.canonicalize()?.into_boxed_path()),
      bin,
      lib,
      edition,
      name,
      registry,
      quite,
      config,
      flags
    }.run().await
  }
}

