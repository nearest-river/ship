
mod app;
mod consts;

#[macro_use]
pub mod macros;
pub mod vcs;
pub mod fs_api;
pub mod config;
pub mod compiler;


use app::App;
use clap::Parser;
use crate::consts::path;
use tracing_subscriber::FmtSubscriber;

use std::{
  env,
  path::Path,
};


#[tokio::main]
async fn main()-> anyhow::Result<()> {
  consts::init();
  FmtSubscriber::builder()
  .compact()
  .with_line_number(false)
  .without_time()
  .with_level(false)
  .with_target(false)
  .init();

  // cwd is gonna change to project root
  
  loop {
    if cfg!(debug_assertions) {
      break;
    }

    match Path::new(path::CONFIG_FILE).try_exists() {
      Ok(true)=> break,
      Ok(false)=> env::set_current_dir("..")?,
      Err(err)=> panic!("no ship project found, reached {err:#?}")
    }
  }

  App::parse().run().await
}

