use std::env;

fn main() {
  if cfg!(feature = "openmp") && cfg!(feature = "tbb") {
    panic!("must choose only one of 'openmp' and 'tbb' features");
  }
  if let Ok(library_paths) = env::var("LIBRARY_PATH") {
    for path in library_paths.split(":") {
      println!("cargo:rustc-link-search=native={}", path);
    }
  }
}
