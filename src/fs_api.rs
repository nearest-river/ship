
mod cwd_mgr;
use tokio::*;
pub use cwd_mgr::*;
use std::path::Path;
use color_print::cformat;

use crate::{
  confirm,
  consts::{
    path,
    event
  }
};




pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P,is_bin: bool)-> io::Result<()> {
  let path=path.as_ref();

  ensure_dir(&path).await?;
  let _cwd_handle=CwdManager::chdir(&path);


  if !path.join(path::CONFIG_FILE).exists() {
    return Ok(());
  }

  let prompt=confirm!(&cformat!("{}: `{}` already consists a ship project. Do you want to continue?",event::WARNING,path.display()),false);

  match prompt {
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display())
    )),
    _=> {
      let _=(// Just ignoring the NotFound errors that may appear..
        fs::remove_file(path::CONFIG_FILE).await,
        fs::remove_file(path::LOCK_FILE).await,
        match is_bin {
          true=> fs::remove_file(path::MAIN).await,
          _=> fs::remove_file(path::LIB).await
        }
      );

      Ok(())
    }
  }
}


pub async fn ensure_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  if !path.as_ref().exists() {
    return fs::create_dir_all(path).await;
  }

  Ok(())
}


