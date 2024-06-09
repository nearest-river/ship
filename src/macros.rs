

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

