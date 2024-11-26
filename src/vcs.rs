use git2::Repository;
use crate::consts::source_code;
use std::{
  path::Path,
  str::FromStr
};

use tokio::{
  fs,
  task,
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



#[derive(Debug,Clone,Copy,Eq)]
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
        task::spawn_blocking(|| Repository::init(".")).await??
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
  pub fn ignore_file(self)-> Option<&'static str> {
    match self {
      Self::Hg=> Some(path::HG_IGNORE),
      Self::Git=> Some(path::GIT_IGNORE),
      Self::Pijul=> Some(path::PIJUL_IGNORE),
      Self::Fossile=> Some(path::FOSSIL_IGNORE),
      Self::None=> None
    }
  }

  pub fn metadata_dir(self)-> Option<Box<Path>> {
    let dir_name=match self {
      Self::Git=> path::GIT_METADATA_DIR,
      Self::Hg=> path::HG_METADATA_DIR,
      Self::Pijul=> path::PIJUL_PIJUL_DIR,
      Self::Fossile=> path::FOSSIL_FOSSIL_DIR,
      Self::None=> None?
    };

    path::PROJECT_ROOT.join(dir_name)
    .into_boxed_path()
    .into()
  }

  pub async fn clone(self,url: impl AsRef<Path>,into: impl AsRef<Path>)-> anyhow::Result<()> {
    let program=match self {
      Self::None=> return Ok(()),
      Self::Fossile=> "fossil",
      Self::Pijul=> "pijul",
      Self::Hg=> "hg",
      Self::Git=> return Self::git_clone(url,into).await
    };

    Command::new(program)
    .arg(url.as_ref().as_os_str())
    .arg(into.as_ref().as_os_str())
    .arg("--recursive")
    .status().await?;

    if self==Self::Hg {
      Command::new("hg")
      .arg("update")
      .status().await?;
    }

    Ok(())
  }

  async fn git_clone(url: impl AsRef<Path>,into: impl AsRef<Path>)-> anyhow::Result<()> {
    let url=url.as_ref().to_owned();
    let into=into.as_ref().to_owned();
    let _xd=task::spawn(async move {
      Repository::clone_recurse(url.to_str().expect("coudln't be parsed"),into)
    }).await?;

    Ok(())
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

