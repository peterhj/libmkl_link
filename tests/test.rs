extern crate cblas_ffi;
extern crate mkl_link;

use cblas_ffi::*;

#[test]
fn test_linking() {
  let x = &[1.0_f32];
  let x_norm = unsafe { cblas_snrm2(1, x.as_ptr(), 1) };
  assert_eq!(1.0, x_norm);
}
