

use tokio::*;
use std::path::Path;
use color_print::cformat;
use crate::prompt::confirm;




pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  let path=path.as_ref();

  // Checks whether there are any files at `path`.
  if let None=fs::read_dir(path).await?.next_entry().await? {
    return Ok(());
  }

  let msg=cformat!("<y>warning</y>: {} is not an empty directory. Do you want to continue?",path.display());
  let prompt=confirm(&msg,false);

  match prompt {
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display())
    )),
    _=> Ok(())
  }
}




