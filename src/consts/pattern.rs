
#[cfg(not(target_os="windows"))]
pub static PATH_SEPERATOR: &str=":";
#[cfg(target_os="windows")]
pub static PATH_SEPERATOR: &str=";";




