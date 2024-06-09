

use tokio::*;
use std::path::Path;
use color_print::cformat;


use crate::{
  confirm,
  consts::{
    paths,
    event
  }
};




pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P,is_bin: bool)-> io::Result<()> {
  std::env::set_current_dir(&path)?;
  let path=path.as_ref();

  match fs::read_dir(path).await {
    Ok(mut res)=> if let None=res.next_entry().await? {
      return Ok(());
    },
    Err(err)=> match err.kind() {
      io::ErrorKind::NotFound=> fs::create_dir_all(path).await?,
      _=> return Err(err)
    }
  }

  let prompt=confirm!(&cformat!("{}: `{}` is not an empty directory. Do you want to continue?",event::WARNING,path.display()),false);

  match prompt {
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display())
    )),
    _=> {
      let _=(// Just ignoring the NotFound errors that may appear..
        fs::remove_dir_all(paths::GIT_REPO_DIR).await,
        fs::remove_file(paths::GITIGNORE).await,
        fs::remove_file(paths::CONFIG_FILE).await,
        fs::remove_file(paths::LOCK_FILE).await,
        match is_bin {
          true=> fs::remove_file(paths::MAIN).await,
          _=> fs::remove_file(paths::LIB).await
        }
      );

      Ok(())
    }
  }
}




