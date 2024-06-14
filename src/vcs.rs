

use std::str::FromStr;
use tokio::process::Command;


use crate::consts::{
  msg,
  event
};

macro_rules! spawn {
  ($command:literal $(,$arg:expr)*)=> {
    Command::new($command)
    .args(["init","--","."])
    $(
      .arg($arg)
    )*
    .status()
    .await?
  };
}



#[derive(Debug,Clone)]
pub enum VersionControl {
  Git,
  Hg,
  Pijul,
  Fossile,
  None
}


impl VersionControl {
  pub async fn init(self)-> anyhow::Result<()> {
    match self {
      VersionControl::Git=> drop(
        tokio::task::spawn_blocking(|| git2::Repository::init(",")).await??
      ),
      VersionControl::Hg=> { spawn!("hg"); },
      VersionControl::Pijul=> { spawn!("pijul"); },
      VersionControl::Fossile=> {
        let db_path = ".fossil";
        let init_code=spawn!("fossil",&db_path);

        // open it in that new directory
        let open_code=Command::new("fossil")
        .args(["open","--"])
        .arg(db_path)
        .status().await?;

        match (init_code.success() && open_code.success(),init_code.code(),open_code.code()) {
          (false,Some(init_code),Some(open_code))=> panik!(code: init_code|open_code,"{}: Failed to initialize fossile repository",event::ERROR),
          _=> {}
        }
      },
      VersionControl::None=> (),
    };

    Ok(())
  }
}

impl FromStr for VersionControl {
  type Err=anyhow::Error;

  fn from_str(vcs: &str)-> anyhow::Result<VersionControl> {
    match vcs {
      "git"=> Ok(Self::Git),
      "hg"=> Ok(Self::Hg),
      "pijul"=> Ok(Self::Pijul),
      "fossil"=> Ok(Self::Fossile),
      "none"=> Ok(Self::None),
      _=> Err(anyhow::Error::msg(msg::VCS_PARSE_ERR))
    }
  }
}

