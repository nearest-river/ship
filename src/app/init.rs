
use tokio::fs;
use clap::Parser;
use color_print::cstr;


use std::{
  path::Path,
  str::FromStr
};

use crate::{
  config::ShipConfig, consts::*, fs_api, panik, vcs::VersionControl
};



#[derive(Parser,Debug)]
pub struct Init {
  #[arg(default_value=".")]
  pub path: Box<Path>,
  #[arg(long,default_value="git")]
  pub vcs: Box<str>,
  #[arg(long)]
  pub bin: bool,
  #[arg(long)]
  pub lib: bool,
  #[arg(long)]
  pub edition: Option<u16>,
  #[arg(long,default_value="")]
  pub name: Box<str>,
  #[arg(long,default_value="todo")]
  pub registry: Box<str>,// TODO(Nate)
  #[arg(long,default_value="false")]
  pub quite: bool,// TODO(Nate)
  #[arg(long,default_value="")]
  pub config: Box<str>,// TODO(Nate)
  #[arg(short='Z',default_value="")]
  pub flags: Box<str>// TODO(Nate)
}


impl Init {
  pub async fn run(mut self)-> anyhow::Result<()> {
    match (self.bin,self.lib) {
      (false,false)=> self.bin=true,
      (true,true)=> panik!(cstr!("<#ff0000>error</#ff0000>: can't specify both lib and binary outputs")),
      _=> {}
    }

    self.path=fs::canonicalize(self.path).await?.into_boxed_path();
    if self.name.is_empty() {
      self.name=self.path.file_name()
      .expect(cstr!("<#ff0000>error</#ff0000>: cannot create a project in the root directory."))
      .to_str().unwrap()
      .into();
    }


    fs_api::ensure_fresh_dir(self.path,self.bin).await?;
    VersionControl::from_str(&self.vcs)?.init(".")?;

    fs::write(paths::GITIGNORE,texts::GITIGNORE).await?;
    fs::create_dir_all(paths::SOURCE_DIR).await?;
    if self.bin {
      fs::write(paths::MAIN,texts::MAIN).await?;
    } else {
      let name=self.name.to_uppercase();
      fs::write(paths::LIB,format!("#ifndef {name}_H\n#define {name}_H\n{}\n#endif\n",texts::LIB)).await?;
    }

    let mut config=ShipConfig::default();

    config.package.name=self.name;
    if let Some(edition)=self.edition {
      config.package.edition=edition;
    }

    config.save(paths::CONFIG_FILE).await?;

    if !self.quite {
      display_summary(self.bin);
    }

    Ok(())
  }
}


fn display_summary(is_bin: bool) {
  let package_type=match is_bin {
    true=> "binary (application)",
    _=> "library"
  };

  tracing::info!("    {} {package_type} package",cstr!("<g,bold>Created</g,bold>"));
  tracing::info!("{}: see more `{}` keys and their definitions at {}",cstr!("<cyan,bold>note</cyan,bold>"),paths::CONFIG_FILE,url::DOCUMENTATION);
}