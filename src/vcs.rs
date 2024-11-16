use std::str::FromStr;
use crate::consts::source_code;

use tokio::{
  fs,
  process::Command
};

use crate::consts::{
  msg,
  path,
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
  None,
  Git,
  Hg,
  Pijul,
  Fossile
}


impl VersionControl {
  pub async fn init(self)-> anyhow::Result<()> {
    match self {
      VersionControl::Git=> drop(
        tokio::task::spawn_blocking(|| git2::Repository::init(".")).await??
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

    if let Some(path)=self.ignore_file() {
      fs::write(path,source_code::VCS_IGNORE).await?;
    }

    Ok(())
  }

  #[inline(always)]
  pub fn ignore_file(&self)-> Option<&'static str> {
    match self {
      Self::Hg=> Some(path::HG_IGNORE),
      Self::Git=> Some(path::GIT_IGNORE),
      Self::Pijul=> Some(path::PIJUL_IGNORE),
      Self::Fossile=> Some(path::FOSSIL_IGNORE),
      Self::None=> None
    }
  }
}

impl FromStr for VersionControl {
  type Err=anyhow::Error;

  #[inline(always)]
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

