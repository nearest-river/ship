use std::{
  env,
  path::Path,
  sync::LazyLock
};


macro_rules! env_var {
  ($key:literal)=> {
    LazyLock::new(|| {
      Box::from(
        Path::new(&std::env::var($key).unwrap())
      )
    })
  };
  ($key:literal ?? $default_val:expr)=> {
    LazyLock::new(|| match std::env::var($key) {
      Ok(val)=> Box::from(Path::new(&val)),
      _=> $default_val
    })
  };
}

pub static HOME: LazyLock<Box<Path>>=env_var!("HOME");
pub static SHIP_INSTALL: LazyLock<Box<Path>>=env_var!("SHIP_INSTALL"??HOME.join(".ship").into_boxed_path());
pub static SHIP_BIN: LazyLock<Box<Path>>=env_var!("SHIP_BIN"??SHIP_INSTALL.join("bin").into_boxed_path());
pub static SHIP_LIB: LazyLock<Box<Path>>=LazyLock::new(|| SHIP_INSTALL.join("lib").into_boxed_path());
pub static INITIAL_WD: LazyLock<Box<Path>>=LazyLock::new(|| {
  env::current_dir()
  .expect("couldn't read cwd")
  .into_boxed_path()
});



pub static CONFIG_FILE: &str="Ship.toml";
pub static LOCK_FILE: &str="Ship.lock";

pub static SOURCE_DIR: &str="src";
pub static GIT_REPO_DIR: &str=".git";
pub static TARGET_DIR: LazyLock<Box<Path>>=env_var!("SHIP_TARGET_DIR"??Path::new("target").into());


pub static LIB_H: &str="src/lib.h";
pub static LIB_C: &str="src/lib.c";
pub static MAIN: &str="src/main.c";

pub static GIT_IGNORE: &str=".gitignore";
pub static HG_IGNORE: &str=".hgignore";
pub static PIJUL_IGNORE: &str=".ignore";
pub static FOSSIL_IGNORE: &str=".fossil-settings/ignore-glob";



