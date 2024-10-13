
use std::{
  path::PathBuf,
  sync::LazyLock
};


macro_rules! env_var {
  ($key:literal)=> {
    LazyLock::new(|| std::env::var($key).unwrap().into())
  };
  ($key:literal ?? $default_val:expr)=> {
    LazyLock::new(|| match std::env::var($key) {
      Ok(val)=> val.into(),
      _=> $default_val.into()
    })
  };
}

pub static HOME: LazyLock<PathBuf>=env_var!("HOME");
pub static SHIP_TARGET_DIR: LazyLock<PathBuf>=env_var!("SHIP_TARGET_DIR"??"target");
pub static SHIP_INSTALL: LazyLock<PathBuf>=env_var!("SHIP_INSTALL"??HOME.join(".ship"));
pub static SHIP_BIN: LazyLock<PathBuf>=env_var!("SHIP_BIN"??SHIP_INSTALL.join("bin"));

