# libmkl_link

This library dynamically links various versions of Intel MKL, depending on the
enabled features. To actually call the MKL BLAS functions, use a CBLAS FFI
crate, such as [cblas_ffi](https://github.com/peterhj/libcblas_ffi).

There are three possible sets of linker args you can invoke:

* sequential MKL (default)
* parallel OpenMP-backed MKL (feature "openmp")
* parallel TBB-backed MKL (feature "tbb"; this pulls in a C++ standard library)

The linked libraries are determined from
<https://software.intel.com/en-us/articles/intel-mkl-link-line-advisor>.
