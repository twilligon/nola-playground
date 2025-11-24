#![deny(improper_ctypes)]

use core::ffi::CStr;
use core::fmt::{Display, Formatter, Result as FmtResult};
use core::ptr::drop_in_place;
use std::error::Error;

#[doc(hidden)]
pub unsafe extern "C-unwind" fn __drop_in_place<T>(ptr: *mut ()) {
    unsafe { drop_in_place(ptr as *mut T) }
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
    pub symbol: &'static CStr,
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "failed to resolve symbol: {}",
            self.symbol.to_string_lossy()
        )
    }
}

impl Error for ResolveError {}

pub mod dyn_trait;
pub use dyn_trait::{Dyn, DynBox, DynRef, DynRefMut, VTable};

#[doc(hidden)]
pub mod abi_safe;

#[doc(hidden)]
pub mod dylib;

#[cfg(unix)]
pub use dylib::__Dylib;
