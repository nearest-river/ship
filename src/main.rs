
mod app;


use app::App;
use clap::Parser;



#[tokio::main]
async fn main() {
  App::parse()
  .run().await
  .unwrap();
}

