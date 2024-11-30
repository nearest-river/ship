use tokio::task;
use std::{
  fs,
  io,
  ffi::OsStr,
  path::Path,
  collections::HashMap,
};




pub struct Fingerprint {
  fingerprints: HashMap<Box<OsStr>,Unit>
}

pub struct Unit;

impl Fingerprint {
  pub async fn open<P: AsRef<Path>>(path: P)-> anyhow::Result<Self> {
    let path=path.as_ref().to_owned();
    let this=task::spawn_blocking(|| {
      Self::open_sync(path)
    })
    .await??;

    anyhow::Ok(this)
  }

  fn open_sync<P: AsRef<Path>>(path: P)-> io::Result<Self> {
    let fingerprints=fs::read_dir(&path)?
    .map(|entry| {
      let entry=entry?;
      if !entry.file_type()?.is_dir() {
        return Ok(None);
      }

      let key=entry.file_name().into_boxed_os_str();
      let unit=Unit;

      io::Result::Ok(Some((key,unit)))
    })
    .filter_map(swap_res_opt)
    .collect::<io::Result<HashMap<_,_>>>()?;

    Ok(Self {
      fingerprints
    })
  }
}

impl Unit {
  // TODO(nate)
  fn is_dirty(&self)-> bool {
    true
  }
}








fn swap_res_opt<T,E>(res: Result<Option<T>,E>)-> Option<Result<T,E>> {
  match res {
    Ok(Some(res))=> Some(Ok(res)),
    Ok(None)=> None,
    Err(err)=> Some(Err(err))
  }
}



