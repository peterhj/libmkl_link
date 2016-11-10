#[cfg(all(not(feature = "openmp"), not(feature = "tbb")))]
pub mod sequential {
  #[link(name = "mkl_intel_lp64", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_sequential", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_core", kind = "static")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}

#[cfg(feature = "openmp")]
pub mod parallel_openmp {
  #[link(name = "mkl_intel_lp64", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_intel_thread", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_core", kind = "static")]
  extern "C" {}

  #[link(name = "iomp5")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}

#[cfg(feature = "tbb")]
pub mod parallel_tbb {
  #[link(name = "mkl_intel_lp64", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_tbb_thread", kind = "static")]
  extern "C" {}

  #[link(name = "mkl_core", kind = "static")]
  extern "C" {}

  #[link(name = "tbb")]
  extern "C" {}

  #[link(name = "stdc++")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}
