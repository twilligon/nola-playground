use core::mem::ManuallyDrop;
use std::vec::Vec as StdVec;

pub use AbiVec as Vec;

#[repr(C)]
pub struct AbiVec<T> {
    cap: usize,
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for AbiVec<T> {
    fn drop(&mut self) {
        unsafe {
            drop(StdVec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

unsafe impl<T> crate::AbiRefSafe for AbiVec<T> {}

unsafe impl<T> crate::AbiSafe<StdVec<T>> for AbiVec<T> {
    fn into_inner(self) -> StdVec<T> {
        let abi = ManuallyDrop::new(self);
        unsafe { StdVec::from_raw_parts(abi.ptr, abi.len, abi.cap) }
    }
}

impl<T> crate::IntoAbiSafe for StdVec<T> {
    type AbiRepr = AbiVec<T>;

    fn into_abi_safe(self) -> AbiVec<T> {
        let vec = ManuallyDrop::new(self);
        AbiVec {
            cap: vec.capacity(),
            ptr: vec.as_ptr() as *mut T,
            len: vec.len(),
        }
    }
}
