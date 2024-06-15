

use std::env;
use clap::Parser;
use std::path::Path;

use tokio::{
  fs,
  io::ErrorKind
};

use crate::{
  skip_handeling,
  config::ShipConfig,
  consts::{
    msg,
    path
  }
};



#[derive(Parser,Debug)]
pub struct Build {
  #[arg(long)]
  ignore_c_version: bool,
  #[arg(long)]
  future_incompat_report: bool,
  #[arg(long)]
  message_format: Option<Box<str>>,
  #[arg(long,short)]
  verbose: bool,
  #[arg(long,short)]
  quite: bool,
  #[arg(long)]
  config: Option<Vec<Box<str>>>,
  #[arg(short='Z')]
  flags: Option<Box<str>>,
  #[arg(long,short)]
  package: Option<Box<str>>,
  #[arg(long)]
  workspace: bool,
  #[arg(long)]
  exclude: Option<Vec<Box<Path>>>,
  #[arg(long)]
  all: bool,
  #[arg(long)]
  lib: bool,
  #[arg(long)]
  bins: bool,
  #[arg(long)]
  bin: Option<Box<str>>,
  #[arg(long)]
  examples: bool,
  #[arg(long)]
  example: Option<Box<Path>>,
  #[arg(long)]
  test: Option<Vec<Box<str>>>,
  #[arg(long)]
  tests: bool,
  #[arg(long)]
  benches: bool,
  #[arg(long)]
  bench: Option<Box<str>>,
  #[arg(long)]
  all_targets: bool,
  #[arg(long,short)]
  release: bool,
  #[arg(long)]
  profile: Option<Box<str>>,
  #[arg(long,short,default_value="default")]
  jobs: Box<str>,// this string is parsed into an int (isize).
  #[arg(long)]
  keep_going: bool,
  #[arg(long)]
  target: Option<Box<str>>,
  #[arg(long)]
  target_dir: Option<Box<Path>>,
  #[arg(long)]
  out_dir: Option<Box<Path>>,
  #[arg(long)]
  build_plan: bool,
  #[arg(long)]
  unit_graph: bool,
  #[arg(long)]
  timings: Option<Box<str>>,
  // Manifest Options
  #[arg(long)]
  manifest_path: Option<Box<Path>>,
  #[arg(long)]
  frozen: bool,
  #[arg(long)]
  locked: bool,
  #[arg(long)]
  offline: bool,
  #[arg(short='C')]
  cwd: Option<Box<Path>>,
}


impl Build {
  // TODO(Nate)
  pub async fn build(self)-> anyhow::Result<()> {
    match (self.cwd,&self.manifest_path) {
      (Some(cwd),_)=> env::set_current_dir(cwd)?,
      (_,Some(path))=> env::set_current_dir(path.parent().expect(msg::INVALID_MANIFEST_PATH))?,
      _=> {}
    }

    let _config=match self.manifest_path {
      None=> ShipConfig::fetch_config().await?,
      Some(path)=> {
        let buf=fs::read_to_string(path).await?;
        toml::from_str(&buf)?
      }
    };

    skip_handeling!(fs::create_dir_all(path::TARGET_DIR).await => ErrorKind::AlreadyExists => Ok(()))?;




    Ok(())
  }
}




