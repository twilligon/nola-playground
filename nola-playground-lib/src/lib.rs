#![deny(improper_ctypes)]

use core::mem::ManuallyDrop;

#[repr(C)]
pub struct VecAbi<T> {
    cap: usize,
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for VecAbi<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

pub fn abi_to_vec<T>(abi: VecAbi<T>) -> Vec<T> {
    let abi = ManuallyDrop::new(abi);
    unsafe { Vec::from_raw_parts(abi.ptr, abi.len, abi.cap) }
}

pub fn vec_to_abi<T>(vec: Vec<T>) -> VecAbi<T> {
    let vec = ManuallyDrop::new(vec);
    VecAbi {
        cap: vec.capacity(),
        ptr: vec.as_ptr() as *mut T,
        len: vec.len(),
    }
}

pub trait Summer {
    fn sum(&mut self, v: Vec<i32>) -> i32;
}

pub mod nola_playground_impl {
    #[repr(C)]
    #[derive(Debug)]
    #[allow(non_snake_case)]
    pub struct __SummerVtable {
        pub drop: Option<unsafe extern "C-unwind" fn(*mut ())>,
        pub size: usize,
        pub align: usize,
        pub __Summer_sum: unsafe extern "C-unwind" fn(*mut (), super::VecAbi<i32>) -> i32,
    }

    #[repr(C)]
    pub struct __SummerDynBox {
        pub data: *mut (),
        pub vtable: *const __SummerVtable,
    }

    impl Drop for __SummerDynBox {
        fn drop(&mut self) {
            unsafe {
                let vtable = &*self.vtable;
                if let Some(drop_fn) = vtable.drop {
                    drop_fn(self.data);
                }
                if vtable.size > 0 {
                    ::std::alloc::dealloc(
                        self.data as *mut u8,
                        ::std::alloc::Layout::from_size_align(vtable.size, vtable.align).unwrap(),
                    );
                }
            }
        }
    }

    impl super::Summer for __SummerDynBox {
        fn sum(&mut self, v: Vec<i32>) -> i32 {
            unsafe { ((*self.vtable).__Summer_sum)(self.data, super::vec_to_abi(v)) }
        }
    }

    fn __resolve(name: &::core::ffi::CStr) -> ::core::ptr::NonNull<::core::ffi::c_void> {
        struct Handle(::core::ptr::NonNull<::core::ffi::c_void>);
        unsafe impl Send for Handle {}
        unsafe impl Sync for Handle {}

        static HANDLE: ::std::sync::LazyLock<Handle> = ::std::sync::LazyLock::new(|| {
            let path = ::std::ffi::CString::new(
                ::std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("deps/libnola_playground_impl.so")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
            let h =
                unsafe { ::libc::dlopen(path.as_ptr(), ::libc::RTLD_LAZY | ::libc::RTLD_LOCAL) };
            Handle(
                ::core::ptr::NonNull::new(h)
                    .unwrap_or_else(|| panic!("dlopen failed: {}", path.to_str().unwrap())),
            )
        });

        let ptr = unsafe { ::libc::dlsym(HANDLE.0.as_ptr(), name.as_ptr()) };
        ::core::ptr::NonNull::new(ptr)
            .unwrap_or_else(|| panic!("dlsym failed: {}", name.to_str().unwrap()))
    }

    #[inline(always)]
    pub fn doubled(v: Vec<i32>) -> Vec<i32> {
        unsafe extern "C-unwind" fn __thunk(v: super::VecAbi<i32>) -> super::VecAbi<i32> {
            let real = __resolve(c"__nola_0_1_0__doubled").as_ptr();
            __FUNC.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn(super::VecAbi<i32>) -> super::VecAbi<i32>,
                >(real))(v)
            }
        }
        static __FUNC: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            super::abi_to_vec((::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn(super::VecAbi<i32>) -> super::VecAbi<i32>,
            >(
                __FUNC.load(::core::sync::atomic::Ordering::Relaxed)
            ))(super::vec_to_abi(v)))
        }
    }

    #[inline(always)]
    pub fn summer1() -> impl super::Summer {
        unsafe extern "C-unwind" fn __thunk() -> __SummerDynBox {
            let real = __resolve(c"__nola_0_1_0__get_summer1").as_ptr();
            __FUNC.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn() -> __SummerDynBox,
                >(real))()
            }
        }
        static __FUNC: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            (::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn() -> __SummerDynBox,
            >(__FUNC.load(::core::sync::atomic::Ordering::Relaxed)))()
        }
    }

    #[inline(always)]
    pub fn summer2() -> impl super::Summer {
        unsafe extern "C-unwind" fn __thunk() -> __SummerDynBox {
            let real = __resolve(c"__nola_0_1_0__get_summer2").as_ptr();
            __FUNC.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn() -> __SummerDynBox,
                >(real))()
            }
        }
        static __FUNC: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            (::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn() -> __SummerDynBox,
            >(__FUNC.load(::core::sync::atomic::Ordering::Relaxed)))()
        }
    }
}
