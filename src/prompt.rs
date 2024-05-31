

#[macro_export]
macro_rules! confirm {
  ($msg:expr,$default:expr)=> {
    inquire::Confirm::new($msg)
    .with_default($default)
    .prompt().unwrap()
  };
}



