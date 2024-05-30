

use tokio::*;
use clap::Parser;
use crate::config::ShipConfig;
use crate::fs_api;
use crate::texts::*;
use std::path::Path;
use color_print::cstr;


#[derive(Parser,Debug)]
pub struct Init {
  #[arg(default_value=".")]
  pub path: Box<Path>,
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
  pub async fn run(mut self)-> io::Result<()> {
    match (self.bin,self.lib) {
      (false,false)=> self.bin=true,
      (true,true)=> panic!(cstr!("<#ff0000>error</#ff0000>: can't specify both lib and binary outputs")),
      _=> {}
    }

    self.path=fs::canonicalize(self.path).await?.into_boxed_path();
    if self.name.is_empty() {
      self.name=self.path.file_name()
      .expect(cstr!("<#ff0000>error</#ff0000>: cannot create a project in the root directory."))
      .to_str().unwrap()
      .into();
    }


    fs_api::ensure_fresh_dir(".",self.bin).await?;
    git2::Repository::init(".").unwrap();

    fs::write(".gitignore",GITIGNORE).await?;
    fs::create_dir_all("src").await?;
    if self.bin {
      fs::write("src/main.c",MAIN_C).await?;
    } else {
      let name=self.name.to_uppercase();
      fs::write("src/lib.c",format!("#ifndef {name}_H\n#define {name}_H\n{LIB_C}\n#endif\n")).await?;
    }

    let mut config=ShipConfig::default();

    config.package.name=self.name;
    if let Some(edition)=self.edition {
      config.package.edition=edition;
    }

    config.save(ShipConfig::CONFIG_FILE_NAME).await?;
    Ok(())
  }
}

