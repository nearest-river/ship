use color_print::cstr;


pub const VCS_PARSE_ERR: &str=cstr!("<#ff0000,bold>error</#ff0000,bold>: invalid value '<#ffff00,bold>xd</#ffff00,bold>' for <#00ffff,bold>--vcs</#00ffff,bold>\n[possible values: <#00ffff,bold>git</#00ffff,bold>, <#00ffff,bold>hg</#00ffff,bold>, <#00ffff,bold>pijul</#00ffff,bold>, <#00ffff,bold>fossil</#00ffff,bold>, <#00ffff,bold>none</#00ffff,bold>].\n\nFor more information, try '<#00ffff,bold>--help</#00ffff,bold>'.");
pub const PROJECT_IN_ROOT_DIR: &str=cstr!("<#ff0000,bold>error</#ff0000,bold>: cannot create a project in the root directory.");



