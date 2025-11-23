#![deny(improper_ctypes)]

pub trait Summer {
    fn sum(&mut self, v: Vec<i32>) -> i32;

    #[doc(hidden)]
    fn __into_dyn_box(self: Box<Self>) -> __SummerDynBox
    where
        Self: Sized,
    {
        __SummerDynBox {
            data: Box::into_raw(self) as *mut (),
            vtable: &const {
                __SummerVtable {
                    drop: if ::core::mem::needs_drop::<Self>() {
                        Some(::nola_abi_playground::__drop_in_place::<Self>)
                    } else {
                        None
                    },
                    size: ::core::mem::size_of::<Self>(),
                    align: ::core::mem::align_of::<Self>(),
                    __Summer_sum: __Summer_sum::<Self>,
                }
            },
        }
    }
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct __SummerVtable {
    pub drop: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pub size: usize,
    pub align: usize,
    pub __Summer_sum: unsafe extern "C-unwind" fn(
        *mut (),
        ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
    ) -> i32,
}

#[doc(hidden)]
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

impl Summer for __SummerDynBox {
    fn sum(&mut self, v: Vec<i32>) -> i32 {
        unsafe {
            ((*self.vtable).__Summer_sum)(
                self.data,
                ::nola_abi_playground::IntoAbiSafe::into_abi_safe(v),
            )
        }
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe extern "C-unwind" fn __Summer_sum<T: Summer>(
    ptr: *mut (),
    v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
) -> i32 {
    unsafe { &mut *(ptr as *mut T) }.sum(::nola_abi_playground::AbiSafe::into_inner(v))
}

pub mod nola_playground_impl {
    static __DYLIB: ::std::sync::LazyLock<::nola_abi_playground::__Dylib> =
        ::std::sync::LazyLock::new(|| {
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
            ::nola_abi_playground::__Dylib::open(&path)
        });

    #[inline(always)]
    pub fn doubled(v: Vec<i32>) -> Vec<i32> {
        unsafe extern "C-unwind" fn __thunk(
            v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
        ) -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32> {
            let real = __DYLIB.symbol(c"__nola_0_1_0__doubled").as_ptr();
            __SYMBOL.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn(
                        ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
                    )
                        -> ::nola_abi_playground::abi_safe::std::vec::Vec<
                        i32,
                    >,
                >(real))(v)
            }
        }

        static __SYMBOL: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            ::nola_abi_playground::AbiSafe::into_inner((::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn(
                    ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
                )
                    -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
            >(
                __SYMBOL.load(::core::sync::atomic::Ordering::Relaxed),
            ))(
                ::nola_abi_playground::IntoAbiSafe::into_abi_safe(v),
            ))
        }
    }

    #[inline(always)]
    pub fn summer1() -> impl super::Summer {
        unsafe extern "C-unwind" fn __thunk() -> super::__SummerDynBox {
            let real = __DYLIB.symbol(c"__nola_0_1_0__get_summer1").as_ptr();
            __SYMBOL.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn() -> super::__SummerDynBox,
                >(real))()
            }
        }

        static __SYMBOL: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            (::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn() -> super::__SummerDynBox,
            >(__SYMBOL.load(::core::sync::atomic::Ordering::Relaxed)))()
        }
    }

    #[inline(always)]
    pub fn summer2() -> impl super::Summer {
        unsafe extern "C-unwind" fn __thunk() -> super::__SummerDynBox {
            let real = __DYLIB.symbol(c"__nola_0_1_0__get_summer2").as_ptr();
            __SYMBOL.store(real, ::core::sync::atomic::Ordering::Relaxed);
            unsafe {
                (::core::mem::transmute::<
                    *const ::core::ffi::c_void,
                    unsafe extern "C-unwind" fn() -> super::__SummerDynBox,
                >(real))()
            }
        }

        static __SYMBOL: ::core::sync::atomic::AtomicPtr<::core::ffi::c_void> =
            ::core::sync::atomic::AtomicPtr::new(__thunk as *mut _);

        unsafe {
            (::core::mem::transmute::<
                *const ::core::ffi::c_void,
                unsafe extern "C-unwind" fn() -> super::__SummerDynBox,
            >(__SYMBOL.load(::core::sync::atomic::Ordering::Relaxed)))()
        }
    }
}
