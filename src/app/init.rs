
use tokio::fs;
use clap::Parser;

use std::{
  path::Path,
  str::FromStr
};

use crate::{
  panik,
  fs_api,
  consts::*,
  config::ShipConfig,
  vcs::VersionControl
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
      (true,true)=> panik!("{}: can't specify both lib and binary outputs",event::ERROR),
      _=> {}
    }

    self.path=fs::canonicalize(self.path).await?.into_boxed_path();
    // self.name will be empty only when path is "/"
    if self.name.is_empty() {
      self.name=self.path.file_name()
      .expect(msg::PROJECT_IN_ROOT_DIR)
      .to_str().unwrap()
      .into();
    }

    fs_api::ensure_fresh_dir(self.path).await?;
    let (vcs_res,src_dir_res)=tokio::join!{
      VersionControl::from_str(&self.vcs)?.init(),
      fs::create_dir_all(path::SOURCE_DIR)
    };
    vcs_res?;
    src_dir_res?;


    if self.bin {
      fs::write(path::MAIN,source_code::MAIN).await?;
    } else {
      let name=self.name.to_uppercase();

      let (res_c,res_h)=tokio::join! {
        fs::write(path::LIB_C,source_code::LIB_C),
        fs::write(path::LIB_H,format!("#ifndef {name}_H\n#define {name}_H\n{}\n#endif\n",source_code::LIB_H))
      };

      res_c.and(res_h)?;
    }

    let mut config=ShipConfig::default();

    config.package.name=self.name;
    config.package.edition=self.edition;
    config.save(path::CONFIG_FILE).await?;

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

  tracing::info!("{} {package_type} package",event::CREATED);
  tracing::info!("{}: see more `{}` keys and their definitions at {}",event::NOTE,path::CONFIG_FILE,url::DOCUMENTATION);
}
