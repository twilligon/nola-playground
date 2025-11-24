#![deny(improper_ctypes)]

pub trait Summer {
    fn sum(&mut self, v: ::std::vec::Vec<i32>) -> i32;

    #[doc(hidden)]
    fn __into_dyn_box(self: ::std::boxed::Box<Self>) -> __SummerDynBox<'static>
    where
        Self: Sized,
    {
        __SummerDynBox {
            data: ::std::boxed::Box::into_raw(self) as *mut (),
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
            phantom: ::core::marker::PhantomData,
        }
    }
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct __SummerVtable {
    pub drop: ::core::option::Option<unsafe extern "C-unwind" fn(*mut ())>,
    pub size: usize,
    pub align: usize,
    pub __Summer_sum: unsafe extern "C-unwind" fn(
        *mut (),
        ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
    ) -> i32,
}

#[doc(hidden)]
#[repr(C)]
pub struct __SummerDynBox<'lib> {
    data: *mut (),
    vtable: *const __SummerVtable,
    phantom: ::core::marker::PhantomData<&'lib dyn Summer>,
}

impl ::core::ops::Drop for __SummerDynBox<'_> {
    fn drop(&mut self) {
        unsafe {
            let vtable = &*self.vtable;
            if let ::core::option::Option::Some(drop_fn) = vtable.drop {
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

impl Summer for __SummerDynBox<'_> {
    fn sum(&mut self, v: ::std::vec::Vec<i32>) -> i32 {
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

#[doc(hidden)]
pub struct __NolaPlaygroundSymbols {
    pub doubled: ::core::sync::atomic::AtomicPtr<()>,
    pub summer1: ::core::sync::atomic::AtomicPtr<()>,
    pub summer2: ::core::sync::atomic::AtomicPtr<()>,
    pub favorite_number: ::core::sync::atomic::AtomicPtr<()>,
}

impl __NolaPlaygroundSymbols {
    pub const fn new_null() -> Self {
        Self {
            doubled: ::core::sync::atomic::AtomicPtr::new(::core::ptr::null_mut()),
            summer1: ::core::sync::atomic::AtomicPtr::new(::core::ptr::null_mut()),
            summer2: ::core::sync::atomic::AtomicPtr::new(::core::ptr::null_mut()),
            favorite_number: ::core::sync::atomic::AtomicPtr::new(::core::ptr::null_mut()),
        }
    }
}

pub trait StaticLibPath {
    type Output: ::core::convert::AsRef<::core::ffi::CStr>;
    fn get_path(&self) -> Self::Output;
}

impl StaticLibPath for &'static ::core::ffi::CStr {
    type Output = &'static ::core::ffi::CStr;
    fn get_path(&self) -> Self::Output {
        self
    }
}

impl<F, T> StaticLibPath for F
where
    F: ::core::ops::Fn() -> T,
    T: ::core::convert::AsRef<::core::ffi::CStr>,
{
    type Output = T;
    fn get_path(&self) -> Self::Output {
        self()
    }
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct __LIB1_NolaPlayground;

impl __LIB1_NolaPlayground {
    #[doc(hidden)]
    #[inline]
    fn __symbols() -> &'static __NolaPlaygroundSymbols {
        unsafe extern "C-unwind" fn __doubled_thunk(
            v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
        ) -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32> {
            let f = __LIB1_NolaPlayground::__doubled_resolve();
            unsafe { f(v) }
        }

        unsafe extern "C-unwind" fn __summer1_thunk() -> __SummerDynBox<'static> {
            let f = __LIB1_NolaPlayground::__summer1_resolve();
            unsafe { f() }
        }

        unsafe extern "C-unwind" fn __summer2_thunk() -> __SummerDynBox<'static> {
            let f = __LIB1_NolaPlayground::__summer2_resolve();
            unsafe { f() }
        }

        unsafe extern "C-unwind" fn __favorite_number_thunk() -> usize {
            let f = __LIB1_NolaPlayground::__favorite_number_resolve();
            unsafe { f() }
        }

        static __SYMBOLS: __NolaPlaygroundSymbols = __NolaPlaygroundSymbols {
            doubled: ::core::sync::atomic::AtomicPtr::new(__doubled_thunk as *mut ()),
            summer1: ::core::sync::atomic::AtomicPtr::new(__summer1_thunk as *mut ()),
            summer2: ::core::sync::atomic::AtomicPtr::new(__summer2_thunk as *mut ()),
            favorite_number: ::core::sync::atomic::AtomicPtr::new(
                __favorite_number_thunk as *mut (),
            ),
        };

        &__SYMBOLS
    }

    fn __dylib() -> &'static ::nola_abi_playground::__Dylib {
        static __DYLIB: ::std::sync::LazyLock<::nola_abi_playground::__Dylib> =
            ::std::sync::LazyLock::new(|| {
                ::nola_abi_playground::__Dylib::open(
                    StaticLibPath::get_path(
                        &(|| c"target/release/deps/libnola_playground_impl.so"),
                    )
                    .as_ref(),
                )
            });

        &__DYLIB
    }

    pub fn __doubled_resolve() -> unsafe extern "C-unwind" fn(
                    ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
                )
    -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32>{
        let ptr = Self::__dylib().symbol(c"__nola_0_1_0__doubled").as_ptr();
        Self::__symbols()
            .doubled
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn(
                    ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
                )
                    -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
            >(ptr)
        }
    }

    pub fn __summer1_resolve() -> unsafe extern "C-unwind" fn() -> __SummerDynBox<'static> {
        let ptr = Self::__dylib()
            .symbol(c"__nola_0_1_0__get_summer1")
            .as_ptr();
        Self::__symbols()
            .summer1
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> __SummerDynBox<'static>,
            >(ptr)
        }
    }

    pub fn __summer2_resolve() -> unsafe extern "C-unwind" fn() -> __SummerDynBox<'static> {
        let ptr = Self::__dylib()
            .symbol(c"__nola_0_1_0__get_summer2")
            .as_ptr();
        Self::__symbols()
            .summer2
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> __SummerDynBox<'static>,
            >(ptr)
        }
    }

    pub fn __favorite_number_resolve() -> unsafe extern "C-unwind" fn() -> usize {
        let ptr = Self::__dylib()
            .symbol(c"__nola_0_1_0__favorite_number")
            .as_ptr();
        Self::__symbols()
            .favorite_number
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe { ::core::mem::transmute::<*mut (), unsafe extern "C-unwind" fn() -> usize>(ptr) }
    }

    pub fn doubled(&self, v: ::std::vec::Vec<i32>) -> ::std::vec::Vec<i32> {
        unsafe {
            ::nola_abi_playground::AbiSafe::into_inner((::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn(
                    ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
                )
                    -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
            >(
                Self::__symbols()
                    .doubled
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))(
                ::nola_abi_playground::IntoAbiSafe::into_abi_safe(v),
            ))
        }
    }

    pub fn summer1(&self) -> impl Summer + '_ {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> __SummerDynBox<'static>,
            >(
                Self::__symbols()
                    .summer1
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))() as __SummerDynBox<'_>
        }
    }

    pub fn summer2(&self) -> impl Summer + '_ {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> __SummerDynBox<'static>,
            >(
                Self::__symbols()
                    .summer2
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))() as __SummerDynBox<'_>
        }
    }

    pub fn favorite_number(&self) -> usize {
        unsafe {
            (::core::mem::transmute::<*mut (), unsafe extern "C-unwind" fn() -> usize>(
                Self::__symbols()
                    .favorite_number
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))()
        }
    }
}

pub struct NolaPlayground {
    dylib: ::nola_abi_playground::__Dylib,
    symbols: __NolaPlaygroundSymbols,
}

impl NolaPlayground {
    pub fn load(path: impl ::core::convert::AsRef<::core::ffi::CStr>) -> Self {
        Self {
            dylib: ::nola_abi_playground::__Dylib::open(path),
            symbols: __NolaPlaygroundSymbols::new_null(),
        }
    }

    fn get_or_resolve(
        &self,
        cache: &::core::sync::atomic::AtomicPtr<()>,
        name: &::core::ffi::CStr,
    ) -> *mut () {
        let mut ptr = cache.load(::core::sync::atomic::Ordering::Relaxed);
        if ptr.is_null() {
            ptr = self.dylib.symbol(name).as_ptr();
            cache.store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        }
        ptr
    }

    pub fn doubled(&self, v: ::std::vec::Vec<i32>) -> ::std::vec::Vec<i32> {
        let ptr = self.get_or_resolve(&self.symbols.doubled, c"__nola_0_1_0__doubled");
        unsafe {
            let f: unsafe extern "C-unwind" fn(
                ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
            )
                -> ::nola_abi_playground::abi_safe::std::vec::Vec<
                i32,
            > = ::core::mem::transmute(ptr);
            ::nola_abi_playground::AbiSafe::into_inner(f(
                ::nola_abi_playground::IntoAbiSafe::into_abi_safe(v),
            ))
        }
    }

    #[doc(hidden)]
    pub fn __doubled_resolve(
        &self,
    ) -> ::core::result::Result<(), ::nola_abi_playground::ResolveError> {
        const NAME: &::core::ffi::CStr = c"__nola_0_1_0__doubled";
        match self.dylib.try_symbol(NAME) {
            ::core::option::Option::Some(p) => {
                self.symbols
                    .doubled
                    .store(p.as_ptr(), ::core::sync::atomic::Ordering::Relaxed);
                ::core::result::Result::Ok(())
            }
            ::core::option::Option::None => {
                ::core::result::Result::Err(::nola_abi_playground::ResolveError { symbol: NAME })
            }
        }
    }

    pub fn summer1(&self) -> impl Summer + '_ {
        let ptr = self.get_or_resolve(&self.symbols.summer1, c"__nola_0_1_0__get_summer1");
        unsafe {
            let f: unsafe extern "C-unwind" fn() -> __SummerDynBox<'static> =
                ::core::mem::transmute(ptr);
            let result: __SummerDynBox<'_> = f();
            result
        }
    }

    #[doc(hidden)]
    pub fn __summer1_resolve(
        &self,
    ) -> ::core::result::Result<(), ::nola_abi_playground::ResolveError> {
        const NAME: &::core::ffi::CStr = c"__nola_0_1_0__get_summer1";
        match self.dylib.try_symbol(NAME) {
            ::core::option::Option::Some(p) => {
                self.symbols
                    .summer1
                    .store(p.as_ptr(), ::core::sync::atomic::Ordering::Relaxed);
                ::core::result::Result::Ok(())
            }
            ::core::option::Option::None => {
                ::core::result::Result::Err(::nola_abi_playground::ResolveError { symbol: NAME })
            }
        }
    }

    pub fn summer2(&self) -> impl Summer + '_ {
        let ptr = self.get_or_resolve(&self.symbols.summer2, c"__nola_0_1_0__get_summer2");
        unsafe {
            let f: unsafe extern "C-unwind" fn() -> __SummerDynBox<'static> =
                ::core::mem::transmute(ptr);
            let result: __SummerDynBox<'_> = f();
            result
        }
    }

    #[doc(hidden)]
    pub fn __summer2_resolve(
        &self,
    ) -> ::core::result::Result<(), ::nola_abi_playground::ResolveError> {
        const NAME: &::core::ffi::CStr = c"__nola_0_1_0__get_summer2";
        match self.dylib.try_symbol(NAME) {
            ::core::option::Option::Some(p) => {
                self.symbols
                    .summer2
                    .store(p.as_ptr(), ::core::sync::atomic::Ordering::Relaxed);
                ::core::result::Result::Ok(())
            }
            ::core::option::Option::None => {
                ::core::result::Result::Err(::nola_abi_playground::ResolveError { symbol: NAME })
            }
        }
    }

    pub fn favorite_number(&self) -> usize {
        let ptr = self.get_or_resolve(
            &self.symbols.favorite_number,
            c"__nola_0_1_0__favorite_number",
        );
        unsafe {
            let f: unsafe extern "C-unwind" fn() -> usize = ::core::mem::transmute(ptr);
            f()
        }
    }

    #[doc(hidden)]
    pub fn __favorite_number_resolve(
        &self,
    ) -> ::core::result::Result<(), ::nola_abi_playground::ResolveError> {
        const NAME: &::core::ffi::CStr = c"__nola_0_1_0__favorite_number";
        match self.dylib.try_symbol(NAME) {
            ::core::option::Option::Some(p) => {
                self.symbols
                    .favorite_number
                    .store(p.as_ptr(), ::core::sync::atomic::Ordering::Relaxed);
                ::core::result::Result::Ok(())
            }
            ::core::option::Option::None => {
                ::core::result::Result::Err(::nola_abi_playground::ResolveError { symbol: NAME })
            }
        }
    }
}

pub static LIB1: __LIB1_NolaPlayground = __LIB1_NolaPlayground;
