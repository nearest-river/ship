
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

  App::parse().run().await
}

