use tokio::task;
use crate::fs_api;
use std::{
  fs,
  io,
  ffi::OsStr,
  path::Path,
  collections::HashMap,
};




pub struct Fingerprints {
  fingerprints: HashMap<Box<OsStr>,Unit>,
  path: Box<Path>
}

pub struct Unit;

impl Fingerprints {
  pub async fn open<P: AsRef<Path>>(path: P)-> anyhow::Result<Self> {
    let path=path.as_ref().to_owned().into_boxed_path();
    let path_clone=path.clone();
    let fingerprints=task::spawn_blocking(|| {
      Self::open_sync(path_clone)
    })
    .await??;

    anyhow::Ok(Self {
      path,
      fingerprints
    })
  }

  fn open_sync<P: AsRef<Path>>(path: P)-> io::Result<HashMap<Box<OsStr>,Unit>> {
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

    Ok(fingerprints)
  }

  pub fn is_dirty<S: AsRef<OsStr>+?Sized>(&self,fingerprint: &S)-> bool {
    match self.fingerprints.get(fingerprint.as_ref()){
      Some(unit)=> unit.is_dirty(),
      _=> true
    }
  }

  // TODO(nate): generate fingerprint
  pub async fn insert<S: AsRef<OsStr>>(&mut self,fingerprint: S,unit: Unit)-> io::Result<()> {
    let fingerprint=fingerprint.as_ref();
    self.fingerprints.insert(fingerprint.into(),unit);
    let path=self.path.join(fingerprint);

    fs_api::ensure_dir(&path).await?;
    Ok(())
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



