mod cwd_mgr;
pub use cwd_mgr::*;
use tokio_stream::wrappers::ReadDirStream;
use futures::{
  future,
  StreamExt
};

use tokio::{
  fs,
  io,
  sync::RwLock
};

use std::{
  sync::Arc,
  path::Path,
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

pub async fn copy_dir_all(src: impl AsRef<Path>,dest: impl AsRef<Path>)-> io::Result<()> {
  let src=fs::canonicalize(src).await?;
  fs::create_dir_all(&dest).await?;
  let dest=fs::canonicalize(dest.as_ref()).await?;

  type Stack=Vec<(Box<Path>,Box<Path>)>;
  let stack: Arc<RwLock<Stack>>=Arc::new(RwLock::new(vec![
    (src.into_boxed_path(),dest.into_boxed_path())
  ]));

  let mut stack_bind=stack.write().await;
  while let Some((src,dest))=stack_bind.pop() {
    let stream=ReadDirStream::new(fs::read_dir(src).await?)
    .map(|entry| async {
      let entry=entry?;
      let mut src_path=entry.path();
      let dest_path=dest.join(src_path.file_name().unwrap());
      let metadata=fs::metadata(&src_path).await?;

      if metadata.is_symlink() {
        src_path=fs::read_link(src_path).await?;
      }

      if metadata.is_dir() {
        fs::create_dir_all(&dest_path).await?;
        let stack_binding=Arc::clone(&stack);
        let mut stack=stack_binding.write().await;
        stack.push((src_path.into_boxed_path(),dest_path.into_boxed_path()));
        return Ok(());
      }

      fs::copy(&src_path,&dest_path).await?;
      io::Result::Ok(())
    }).collect::<Vec<_>>().await;

    for res in future::join_all(stream).await {
      res?;
    }

    stack_bind=stack.write().await;
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use std::io;

  #[tokio::test]
  async fn copy_dir_all()-> io::Result<()> {
    super::copy_dir_all("/home/nate/Games/","/home/nate/temp_games").await
  }
}





