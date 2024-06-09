
use clap::Parser;

use tokio::{
  fs,
  io
};

use crate::{
  consts::event,
  consts::path,
  skip_handeling,
  config::ShipConfig
};

use std::{
  path::Path,
  collections::VecDeque,
};

#[cfg(not(target_os="windows"))]
use std::os::unix::fs::MetadataExt;



#[derive(Parser,Debug)]
pub struct Clean {
  #[arg(long)]
  doc: bool,
  #[arg(long,short)]
  dry_run: bool,
  #[arg(long,short)]
  quite: bool
}


impl Clean {
  pub async fn clean(self)-> anyhow::Result<()> {
    drop(ShipConfig::fetch_config().await?);// just to make sure that root of the project is the cwd.
    let mut count=0usize;
    let mut size=0u64;
    let mut queue=VecDeque::<Box<Path>>::new();

    if self.doc {
      queue.push_back(Path::new(path::DOC_DIR).into());
    }
    queue.push_back(Path::new(path::TARGET_DIR).into());


    while let Some(path)=queue.pop_back() {
      let mut iter=skip_handeling!(fs::read_dir(&path).await => io::ErrorKind::NotFound => continue)?;

      while let Some(entry)=iter.next_entry().await? {
        let metadata=entry.metadata().await?;
        if metadata.is_dir() {
          queue.push_back(entry.path().into_boxed_path());
          continue;
        }

        count+=1;
        size+=metadata.size();
      }
    }

    let msg=match self.dry_run {
      true=> event::SUMMARY,
      _=> {
        skip_handeling!(fs::remove_dir_all(path::TARGET_DIR).await => io::ErrorKind::NotFound => Ok(()))?;
        event::REMOVED
      }
    };

    if !self.quite {
      self.display_summary(msg,size,count);
    }

    Ok(())
  }

  fn display_summary(&self,msg: &str,size: u64,count: usize) {
    let txt=match size {
      1024.. => {
        let (size,unit)=human_readable_bytes(size);
        color_print::cformat!("{msg} {count} files, {size:.1}{unit} total")
      },
      _=> color_print::cformat!("{msg} {count} files, {size}B total")
    };

    tracing::info!("{txt}");
  }
}


fn human_readable_bytes(bytes: u64)-> (f32,&'static str) {
  pub const UNITS: [&'static str;7]=["B","KiB","MiB","GiB","TiB","PiB","EiB"];
  let bytes=bytes as f32;
  let i=((bytes.log2()/10.) as usize).min(UNITS.len()-1);

  (bytes/1024_f32.powi(i as _),UNITS[i])
}



