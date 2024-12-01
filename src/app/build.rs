use clap::Parser;
use futures::future;

use std::{
  mem,
  ffi::OsStr,
  path::Path,
};

use tokio::{
  fs,
  process::Command,
  io::{
    self,
    Error,
    ErrorKind
  }
};

use crate::{
  fs_api::{
    self,
    CwdManager
  },
  config::{
    ShipConfig,
    PackageEntry,
    fingerprint::{
      Unit,
      Fingerprints
    }
  },
  consts::{
    env,
    msg,
    path,
    pattern
  }
};



#[derive(Parser,Debug)]
pub struct Build {
  #[arg(long)]
  ignore_c_version: bool,
  #[arg(long)]
  future_incompat_report: bool,
  #[arg(long)]
  message_format: Option<Box<str>>,
  #[arg(long,short)]
  verbose: bool,
  #[arg(long,short)]
  quite: bool,
  #[arg(long)]
  config: Option<Vec<Box<str>>>,
  #[arg(short='Z')]
  flags: Option<Box<str>>,
  #[arg(long,short)]
  package: Option<Box<str>>,
  #[arg(long)]
  exclude: Option<Vec<Box<Path>>>,
  #[arg(long,short)]
  release: bool,
  #[arg(long)]
  profile: Option<Box<str>>,
  #[arg(long)]
  target: Option<Box<str>>,
  #[arg(long)]
  target_dir: Option<Box<Path>>,
  #[arg(long)]
  out_dir: Option<Box<Path>>,
  #[arg(long)]
  timings: Option<Box<str>>,
  // Manifest Options
  #[arg(long)]
  frozen: bool,
  #[arg(long)]
  locked: bool,
  #[arg(long)]
  offline: bool,
}


impl Build {
  // TODO(Nate): save fingerprints to disk. compile the root crate
  pub async fn build(&self)-> anyhow::Result<()> {
    let config=ShipConfig::open(&path::CONFIG_FILE).await
    .expect(msg::INVALID_MANIFEST_PATH);
    let build_profile=match self.release {
      true=> "release",
      _=> "debug"
    };
    let target_dir=self.target_dir
    .as_deref()
    .unwrap_or(&path::TARGET_DIR)
    .join(build_profile);
    let deps_dir=target_dir.join("deps");
    fs_api::ensure_dir(&deps_dir).await?;

    let pkg_lock=config.dependencies.sync(&target_dir).await?;
    let fingerprints=Fingerprints::open(target_dir.join(".fingerprints")).await?;

    let stream=pkg_lock.packages.into_iter()
    .map(|pkg| {
      let fingerprints=&fingerprints;
      self.compile_deps(pkg,&deps_dir,fingerprints)
    });

    let _deltas=future::join_all(stream).await;

    Ok(())
  }

  // generate fingerprints
  async fn compile_deps(&self,pkg: PackageEntry,deps_dir: &Path,fingerprints: &Fingerprints)-> io::Result<(Box<OsStr>,Unit)> {
    if fingerprints.is_dirty(OsStr::new(&*pkg.name)) {
      return Ok((OsStr::new(&*pkg.name).into(),Unit));
    }

    let out_dir=deps_dir.join(&*pkg.name);
    let registered_name=format!("{}-{}",pkg.name,pkg.version);
    let _=CwdManager::chdir(&out_dir);

    fs::create_dir_all(out_dir).await?;
    let source_file_paths=Self::source_files(&registered_name,&deps_dir).await?;
    let mut compiler_options=vec!["-Wall","-c"]; // FIXME
    if !self.release {
      compiler_options.push("-g");
    }

    Self::compile(&source_file_paths,compiler_options).await?;
    let objects=source_file_paths.into_iter()
    .map(|path| {
      let path=Path::new(&*path).with_extension("o");
      path.file_name().unwrap().to_owned()
    });

    Self::link(&*pkg.name,objects).await?;
    Ok((OsStr::new(&*pkg.name).into(),Unit))
  }

  async fn compile<S: AsRef<OsStr>>(source_files: &[S],compiler_options: Vec<&str>)-> io::Result<()> {
    let mut process=Command::new(&*env::CC);
    process.args(compiler_options);
    process.args(source_files);

    if process.status().await?.success() {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Other,"compilation failed"))
    }
  }

  async fn link<S: AsRef<OsStr>>(lib_name: &str,objects: impl Iterator<Item=S>)-> io::Result<()> {
    let mut process=Command::new(&*env::LINKER);
    process.arg("-rc");
    process.arg(format!("lib{lib_name}.{}",path::STATIC_LIBRARY_EXTENTION));
    process.args(objects);

    if process.status().await?.success() {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Other,"linking failed"))
    }
  }

  async fn source_files(registered_name: &str,deps_dir: impl AsRef<Path>)-> io::Result<Vec<Box<OsStr>>> {
    let d_path=deps_dir.as_ref().join(&registered_name);
    let dep_d=match fs::read_to_string(&d_path).await {
      Ok(string)=> string,
      Err(err) if err.kind()==ErrorKind::NotFound => {
        return Self::extract_deps(registered_name,d_path).await
      },
      err=> err?
    };

    let paths=dep_d.split(pattern::PATH_SEPERATOR)
    .map(|path| Box::from(OsStr::new(path)))
    .collect::<Vec<_>>();

    Ok(paths)
  }

  async fn extract_deps<P: AsRef<Path>>(registered_name: &str,d_path: P)-> io::Result<Vec<Box<OsStr>>> {
    let src_path=path::SHIP_LIB.join(registered_name);
    let mut stack=vec![Box::from(src_path)];
    let mut source_paths=Vec::new();

    while let Some(path)=stack.pop() {
      let mut stream=fs::read_dir(path).await?;

      while let Some(entry)=stream.next_entry().await? {
        let file_type=entry.file_type().await?;
        let path=entry.path().into_boxed_path();

        if file_type.is_dir() {
          stack.push(path);
          continue;
        }

        if file_type.is_file() && path.ends_with(".c") {
          // both have the same memory layout
          let path=unsafe {
            mem::transmute::<_,Box<OsStr>>(path)
          };
          source_paths.push(path);
        }
      }
    }

    let buf=source_paths.join(pattern::PATH_SEPERATOR.as_ref()).into_encoded_bytes();
    fs::write(d_path,buf).await?;
    Ok(source_paths)
  }
}




