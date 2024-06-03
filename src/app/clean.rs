
use tokio::*;
use clap::Parser;
use crate::consts::paths;

use std::{
  path::Path,
  collections::VecDeque,
};

#[cfg(not(target_os="windows"))]
use std::os::unix::fs::MetadataExt;



#[derive(Parser,Debug)]
pub struct Clean;


impl Clean {
  pub async fn clean(self)-> io::Result<()> {
    let mut count=0usize;
    let mut size=0u64;
    let mut queue=VecDeque::<Box<Path>>::new();
    queue.push_back(Path::new(paths::TARGET_DIR).into());

    while let Some(path)=queue.pop_back() {
      let mut iter=fs::read_dir(&path).await?;

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
    fs::remove_dir_all(paths::TARGET_DIR).await?;

    color_print::cprintln!("\t<bold><g>Removed</g></bold> {count} files, {size} bytes total");
    Ok(())
  }
}

