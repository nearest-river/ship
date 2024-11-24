
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
  sync::LazyLock
};

pub static INITIAL_WD: LazyLock<Box<Path>>=LazyLock::new(|| {
  env::current_dir()
  .expect("couldn't read cwd")
  .into_boxed_path()
});


#[tokio::main]
async fn main()-> anyhow::Result<()> {
  tracing::info!("main");
  FmtSubscriber::builder()
  .compact()
  .with_line_number(false)
  .without_time()
  .with_level(false)
  .with_target(false)
  .init();

  tracing::info!("xd");
  // cwd is gonna change to project root
  LazyLock::force(&INITIAL_WD);

  loop {
    break;
    match Path::new(path::CONFIG_FILE).try_exists() {
      Ok(true)=> break,
      Ok(false)=> env::set_current_dir("..")?,
      Err(err)=> panic!("no ship project found, reached {err:#?}")
    }
  }

  App::parse().run().await
}

