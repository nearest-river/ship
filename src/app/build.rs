

use clap::Parser;
use std::path::Path;


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
  config: Option<Box<str>>,
  #[arg(long,short='Z')]
  flags: Option<Box<str>>,
  #[arg(long,short)]
  help: bool,
  #[arg(long,short)]
  package: Option<Box<str>>,
  #[arg(long)]
  workspace: bool,
  #[arg(long)]
  exclude: Option<Box<str>>,
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
  tests: Option<Box<str>>,
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
  #[arg(long,short)]
  jobs: usize,
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
  offline: bool
}


impl Build {
  // TODO(Nate)
  pub async fn build(self)-> anyhow::Result<()> {
    Ok(())
  }
}




