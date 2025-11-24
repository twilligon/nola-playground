#![deny(improper_ctypes)]

#[doc(hidden)]
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

#[derive(Debug)]
pub struct ResolveError {
    pub symbol: &'static ::core::ffi::CStr,
}

impl ::core::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "failed to resolve symbol: {}",
            self.symbol.to_string_lossy()
        )
    }
}

impl ::std::error::Error for ResolveError {}

#[doc(hidden)]
pub mod abi_safe;

#[doc(hidden)]
pub mod dylib;

#[cfg(unix)]
pub use dylib::__Dylib;
