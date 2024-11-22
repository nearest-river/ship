
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
use tokio::sync::OnceCell;
use tracing_subscriber::FmtSubscriber;

use std::{
  env,
  path::Path,
};

pub static INITIAL_WD: OnceCell<Box<Path>>=OnceCell::const_new();


#[tokio::main]
async fn main()-> anyhow::Result<()> {
  FmtSubscriber::builder()
  .compact()
  .with_line_number(false)
  .without_time()
  .with_level(false)
  .with_target(false)
  .init();

  // cwd is gonna change to project root
  INITIAL_WD.get_or_init(|| async {
    env::current_dir()
    .expect("couldn't read cwd")
  }).await;

  loop {
    match Path::new(path::CONFIG_FILE).try_exists() {
      Ok(true)=> break,
      Ok(false)=> env::set_current_dir("..")?,
      Err(err)=> panic!("no ship project found, reached {err:#?}")
    }
  }

  App::parse().run().await
}

