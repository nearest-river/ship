

#[macro_export]
macro_rules! confirm {
  ($msg:expr,$default:expr)=> {
    inquire::Confirm::new($msg)
    .with_default($default)
    .prompt().unwrap()
  };
}

#[macro_export]
macro_rules! skip_handeling {
  ($res:expr => $skip:pat => $token:expr)=> {
    {
      let res=$res;
      match res {
        Ok(res)=> Ok(res),
        Err(err)=> match err.kind() {
          $skip=> $token,
          _=> Err(err)
        }
      }
    }
  };
}


#[macro_export]
macro_rules! panik {
  (code: $code:expr,$($arg:tt)*)=> {{
    tracing::error!($($arg)*);
    std::process::exit($code);
  }};
  ($($arg:tt)*)=> {{
    tracing::error!($($arg)*);
    std::process::exit(1);
  }};
}

#[macro_export]
macro_rules! progress_style {
  ()=> {
    indicatif::ProgressStyle::with_template(
      if console::Term::stdout().size().1 > 80 {
        "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
      } else {
        "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
      },
    )
    .unwrap()
    .progress_chars("=> ")
  };
}


