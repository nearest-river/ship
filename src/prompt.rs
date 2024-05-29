
use requestty::Question;


pub fn confirm(msg: &str,default: bool)-> bool {
  let q=Question::confirm(msg)
  .default(default)
  .build();

  match requestty::prompt_one(q) {
    Ok(res)=> res.as_bool().unwrap(),
    _=> default
  }
}


