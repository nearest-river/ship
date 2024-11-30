use std::{
  env,
  sync::LazyLock
};


pub static DEFAULT_CC: &str="gcc";
pub static CC: LazyLock<String>=LazyLock::new(|| {
  env::var("CC")
  .unwrap_or_else(|_| DEFAULT_CC.to_owned())
});

pub static DEFAULT_LINKER: &str="ar";
pub static LINKER: LazyLock<String>=LazyLock::new(|| {
  env::var("LINKER")
  .unwrap_or_else(|_| DEFAULT_LINKER.to_owned())
});





