
pub static VCS_IGNORE: &str=r#"target/
*.lock
"#;

pub static MAIN: &str=r#"#include <stdio.h>

int main(int argc,const char** argv) {
  printf("Hello, World!\n");
  return 0;
}
"#;

pub static LIB: &str=r#"
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

