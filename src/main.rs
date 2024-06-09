
mod app;
mod consts;

pub mod config;
pub mod fs_api;
#[macro_use]
pub mod macros;
pub mod vcs;


use app::App;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;


#[tokio::main]
async fn main() {
  FmtSubscriber::builder()
  .compact()
  .with_line_number(false)
  .without_time()
  .with_level(false)
  .with_target(false)
  .init();

  match App::parse().run().await {
    Ok(_)=> todo!(),
    Err(err)=> {
      // tracing::error!("{err}");
      std::process::exit(err.downcast_ref::<i32>().cloned().unwrap_or(1));
    },
  }
}

