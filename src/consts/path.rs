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
pub static PROJECT_ROOT: LazyLock<Box<Path>>=LazyLock::new(|| {
  // cwd is gonna change to project root
  loop {
    match Path::new(CONFIG_FILE).try_exists() {
      Ok(true)=> break,
      Ok(false)=> env::set_current_dir("..").expect("couldn't access the filesystem"),
      Err(err)=> panic!("no ship project found, reached {err:#?}")
    }
  }

  std::fs::canonicalize(".")
  .expect("couldn't access the filesystem")
  .into_boxed_path()
});


pub static SOURCE_DIR: &str="src";
pub static TARGET_DIR: LazyLock<Box<Path>>=env_var!("SHIP_TARGET_DIR"??Path::new("target").into());



pub static LIB_H: &str="src/lib.h";
pub static LIB_C: &str="src/lib.c";
pub static MAIN: &str="src/main.c";

pub static GIT_IGNORE: &str=".gitignore";
pub static HG_IGNORE: &str=".hgignore";
pub static PIJUL_IGNORE: &str=".ignore";
pub static FOSSIL_IGNORE: &str=".fossil-settings/ignore-glob";

pub static GIT_METADATA_DIR: &str=".git";
pub static HG_METADATA_DIR: &str=".hg";
pub static PIJUL_PIJUL_DIR: &str=".pijul";
pub static FOSSIL_FOSSIL_DIR: &str=".fossil";


#[cfg(not(target_os="windows"))]
pub static STATIC_LIBRARY_EXTENTION: &str="a";
#[cfg(target_os="windows")]
pub static STATIC_LIBRARY_EXTENTION: &str="lib";

#[cfg(not(all(target_os="windows",target_os="macos",target_os="ios")))]
pub static SHARED_LIBRARY_EXTENTION: &str="so";
#[cfg(target_os="windows")]
pub static SHARED_LIBRARY_EXTENTION: &str="dll";
#[cfg(any(target_os="macos",target_os="ios"))]
pub static SHARED_LIBRARY_EXTENTION: &str="dll";




