mod cwd_mgr;
pub use cwd_mgr::*;

use std::{
  path::Path,
  future::Future
};

use tokio::{
  fs,
  io,
};

use crate::{
  confirm,
  consts::{
    path,
    event
  }
};




pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  let path=path.as_ref();

  ensure_dir(&path).await?;
  let _cwd_handle=CwdManager::chdir(&path);


  if !path.join(path::CONFIG_FILE).exists() {
    return Ok(());
  }

  let prompt=confirm!(&format!("{}: `{}` already consists a ship project. Do you want to continue?",event::WARNING,path.display()),false);

  match prompt {
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display())
    )),
    _=> {
      let _=(// Just ignoring the NotFound errors that may appear..
        fs::remove_file(path::CONFIG_FILE).await,
        fs::remove_file(path::LOCK_FILE).await,
        fs::remove_file(path::MAIN).await,
        fs::remove_file(path::LIB_C).await,
        fs::remove_file(path::LIB_H).await
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

#[inline(always)]
pub fn ignore_notfound<T>(result: io::Result<T>)-> io::Result<()> {
  match result {
    Err(err) if err.kind()==io::ErrorKind::NotFound=> Ok(()),
    Ok(_)=> Ok(()),
    Err(err)=> Err(err),
  }
}

pub async fn copy_dir_all(from: impl AsRef<Path>,to: impl AsRef<Path>)-> io::Result<()> {
  let from=Box::<Path>::from(from.as_ref());
  let to=Box::<Path>::from(to.as_ref());
  let mut stack=vec![(from,to)];

  while let Some((from,to))=stack.pop() {
    let mut stream=fs::read_dir(from).await?;

    while let Some(entry)=stream.next_entry().await? {
      let file_type=entry.file_type().await?;
      let mut src=entry.path();
      let dest=to.join(entry.file_name());

      if file_type.is_symlink() {
        src=fs::read_link(src).await?;
      }

      if file_type.is_dir() {
        stack.push((src.into_boxed_path(),dest.into_boxed_path()));
        continue;
      }

      fs::copy(src,dest).await?;
    }
  }

  Ok(())
}




