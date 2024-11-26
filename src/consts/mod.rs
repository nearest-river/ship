
pub mod url;
pub mod msg;
pub mod path;
pub mod event;
pub mod source_code;


pub fn init() {
  use std::sync::LazyLock;

  LazyLock::force(&path::HOME);
  LazyLock::force(&path::SHIP_INSTALL);
  LazyLock::force(&path::SHIP_LIB);
  LazyLock::force(&path::SHIP_BIN);
  LazyLock::force(&path::TARGET_DIR);
  LazyLock::force(&path::PROJECT_ROOT);
  LazyLock::force(&path::INITIAL_WD);
}

