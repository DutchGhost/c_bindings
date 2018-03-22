extern crate gcc;

#[cfg(target_os = "linux")]
fn build() {
    gcc::compile_library("c_bindings", &["src/c/Linux.c"]);
}

 #[cfg(target_os = "windows")]
 fn build() {
    gcc::compile_library("c_bindings", &["src/c/Windows.c"]);
 }

fn main() {
    build();
}