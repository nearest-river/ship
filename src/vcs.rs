

use std::str::FromStr;
use crate::consts::msg;


#[derive(Debug,Clone)]
pub enum VersionControl {
  Git,
  Hg,
  Pijul,
  Fossile,
  None
}


impl VersionControl {
  pub fn init<P: AsRef<std::path::Path>>(self,path: P)-> anyhow::Result<()> {
    match self {
      VersionControl::Git=> { git2::Repository::init(path)?; },
      VersionControl::Hg=> todo!(),
      VersionControl::Pijul=> todo!(),
      VersionControl::Fossile=> todo!(),
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

