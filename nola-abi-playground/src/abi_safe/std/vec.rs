use core::mem::ManuallyDrop;

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
            drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

unsafe impl<T> crate::AbiRefSafe for AbiVec<T> {}

unsafe impl<T> crate::AbiSafe<Vec<T>> for AbiVec<T> {
    fn into_inner(self) -> Vec<T> {
        let abi = ManuallyDrop::new(self);
        unsafe { Vec::from_raw_parts(abi.ptr, abi.len, abi.cap) }
    }
}

impl<T> crate::IntoAbiSafe for Vec<T> {
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
