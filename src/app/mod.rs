

use tokio::io;
use clap::Parser;


#[derive(Parser,Debug)]
pub enum App {
  New,
  Init,
  Clean,
  Check,
  Build,
  Run,
  Test,
  Bench,
  Add,
  Remove,
  Install,
  Uninstall,
  Publish,
  Doc,
  Help
}


impl App {
  pub async fn run(self)-> io::Result<()> {
    Ok(())
  }
}




