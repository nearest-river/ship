
mod app;
mod consts;

pub mod config;
pub mod fs_api;
#[macro_use]
pub mod macros;


use app::App;
use clap::Parser;



#[tokio::main]
async fn main() {
  App::parse()
  .run().await
  .unwrap();
}

