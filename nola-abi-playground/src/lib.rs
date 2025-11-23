#![deny(improper_ctypes)]

#[doc(hidden)]
#[inline]
pub unsafe extern "C-unwind" fn __drop_in_place<T>(ptr: *mut ()) {
    unsafe { ::core::ptr::drop_in_place(ptr as *mut T) }
}

pub unsafe trait AbiRefSafe {}

pub unsafe trait AbiSafe<T>: AbiRefSafe {
    fn into_inner(self) -> T;
}

pub trait IntoAbiSafe: Sized {
    type AbiRepr: AbiSafe<Self>;
    fn into_abi_safe(self) -> Self::AbiRepr;
}

#[doc(hidden)]
pub mod abi_safe;

#[doc(hidden)]
pub mod dylib;

#[cfg(unix)]
pub use dylib::__Dylib;
