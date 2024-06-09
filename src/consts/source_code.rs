
pub const GITIGNORE: &str=r#"
target/
"#;

pub const MAIN: &str=r#"
#include <stdio.h>

int main(int argc,const char** argv) {
  printf("Hello, World!");
  return 0;
}
"#;

pub const LIB: &str=r#"
#ifdef _cplusplus
extern "C" {
#endif

int add(int x,int y) {
  return x + y;
}

#ifdef _cplusplus
}
#endif
"#;

