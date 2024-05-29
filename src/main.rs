
mod app;
pub mod config;
pub mod fs_api;
pub mod prompt;


use app::App;
use clap::Parser;



#[tokio::main]
async fn main() {
  App::parse()
  .run().await
  .unwrap();
}

