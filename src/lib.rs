pub use link_impl::*;

#[cfg(all(not(feature = "openmp"), not(feature = "tbb")))]
pub mod link_impl {
  #[link(name = "mkl_intel_lp64")]
  extern "C" {}

  #[link(name = "mkl_sequential")]
  extern "C" {}

  #[link(name = "mkl_core")]
  extern "C" {}

  #[link(name = "pthread")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}

#[cfg(feature = "openmp")]
pub mod link_impl {
  #[link(name = "mkl_intel_lp64")]
  extern "C" {}

  #[link(name = "mkl_intel_thread")]
  extern "C" {}

  #[link(name = "mkl_core")]
  extern "C" {}

  #[link(name = "iomp5")]
  extern "C" {}

  #[link(name = "pthread")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}

#[cfg(feature = "tbb")]
pub mod link_impl {
  #[link(name = "mkl_intel_lp64")]
  extern "C" {}

  #[link(name = "mkl_tbb_thread")]
  extern "C" {}

  #[link(name = "mkl_core")]
  extern "C" {}

  #[link(name = "tbb")]
  extern "C" {}

  #[link(name = "stdc++")]
  extern "C" {}

  #[link(name = "pthread")]
  extern "C" {}

  #[link(name = "m")]
  extern "C" {}

  #[link(name = "dl")]
  extern "C" {}
}
