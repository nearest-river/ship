
use tokio::*;
use clap::Parser;
use std::path::Path;


#[derive(Parser,Debug)]
pub struct Init {
  pub path: Option<Box<Path>>,
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


impl Init {
  pub async fn run(self)-> io::Result<()> {
    Ok(())
  }
}

