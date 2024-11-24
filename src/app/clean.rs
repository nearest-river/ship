use clap::Parser;
use indicatif::ProgressBar;

use tokio::{
  fs,
  io::ErrorKind
};

use std::{
  path::Path,
  fs::Metadata,
};

use crate::{
  progress_style,
  skip_handeling,
  fs_api::ignore_notfound,
  consts::{
    event,
    path
  }
};


#[derive(Parser,Debug)]
pub struct Clean {
  #[arg(long,short)]
  dry_run: bool,
  #[arg(long,short)]
  quite: bool
}


impl Clean {
  pub async fn clean(mut self)-> anyhow::Result<()> {
    let mut count=0usize;
    let mut size=0u64;
    let mut queue=vec![Box::from(Path::new(path::TARGET_DIR))];

    let pb=ProgressBar::new(32)
    .with_style(progress_style!());
    pb.set_prefix("Cleaning");

    if cfg!(debug_assertions) {
      self.dry_run=true;
    }

    
    while let Some(path)=queue.pop() {
      let mut iter=skip_handeling!(fs::read_dir(&path).await => ErrorKind::NotFound => continue)?;

      while let Some(entry)=iter.next_entry().await? {
        let metadata=entry.metadata().await?;

        size+=file_size(&metadata);
        count+=1;
        pb.inc_length(1);
        if metadata.is_dir() {
          queue.push(entry.path().into_boxed_path());
          continue;
        }

        if !self.dry_run {
          ignore_notfound(fs::remove_file(entry.path()).await)?;
        }

        pb.inc(1);
      }
    }

    if cfg!(debug_assertions) {
      pb.finish();
    } else {
      pb.finish_and_clear();
    }

    let msg=match self.dry_run {
      true=> event::SUMMARY,
      _=> {
        ignore_notfound(fs::remove_dir_all(path::TARGET_DIR).await)?;
        event::REMOVED
      }
    };

    if !self.quite {
      self.display_summary(msg,size,count);
    }

    Ok(())
  }

  fn display_summary(&self,msg: &str,size: u64,count: usize) {
    match size {
      1024.. => {
        let (size,unit)=human_readable_bytes(size);
        tracing::info!("{msg} {count} files, {size:.1}{unit} total")
      },
      _=> tracing::info!("{msg} {count} files, {size}B total")
    }
  }
}


fn human_readable_bytes(bytes: u64)-> (f32,&'static str) {
  static UNITS: [&'static str;7]=["B","KiB","MiB","GiB","TiB","PiB","EiB"];
  let bytes=bytes as f32;
  let i=((bytes.log2()/10.) as usize).min(UNITS.len()-1);

  (bytes/1024_f32.powi(i as _),UNITS[i])
}

fn file_size(metadata: &Metadata)-> u64 {
  #[cfg(not(target_os="windows"))]
  return std::os::unix::fs::MetadataExt::size(metadata);
  #[cfg(target_os="windows")]
  return std::os::windows::fs::MetadataExt::file_size(metadata);
}

