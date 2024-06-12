use color_print::cstr;


pub const VCS_PARSE_ERR: &str=cstr!("<r,bold>error</r,bold>: invalid value '<y,bold>xd</y,bold>' for <cyan,bold>--vcs</cyan,bold>\n[possible values: <cyan,bold>git</cyan,bold>, <cyan,bold>hg</cyan,bold>, <cyan,bold>pijul</cyan,bold>, <cyan,bold>fossil</cyan,bold>, <cyan,bold>none</cyan,bold>].\n\nFor more information, try '<cyan,bold>--help</cyan,bold>'.");
pub const PROJECT_IN_ROOT_DIR: &str=cstr!("<r,bold>error</r,bold>: cannot create a project in the root directory.");
pub const INVALID_MANIFEST_PATH: &str=cstr!("<r,bold>error</r,bold>: Invalid Manifest path");


