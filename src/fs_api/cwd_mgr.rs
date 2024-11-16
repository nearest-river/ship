
use std::{
  io,
  env,
  path::{
    Path,
    PathBuf
  }
};



pub struct CwdManager {
  prev_cwd: PathBuf
}

impl CwdManager {
  pub fn chdir<P: AsRef<Path>>(path: P)-> io::Result<Self> {
    let prev_cwd=env::current_dir()?;
    env::set_current_dir(path)?;
    Ok(Self {
      prev_cwd
    })
  }
}

impl Drop for CwdManager {
  fn drop(&mut self) {
    drop(env::set_current_dir(&self.prev_cwd));
  }
}


