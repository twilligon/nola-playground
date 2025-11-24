#![deny(improper_ctypes)]

pub trait Summer {
    fn sum(&mut self, v: ::std::vec::Vec<i32>) -> i32;
}

#[repr(C)]
pub struct DynSummer {
    sum: unsafe extern "C-unwind" fn(
        *mut (),
        ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
    ) -> i32,
}

impl DynSummer {
    pub fn vtable<T: Summer>() -> &'static ::nola_abi_playground::VTable<Self> {
        unsafe extern "C-unwind" fn __sum<T: Summer>(
            ptr: *mut (),
            v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
        ) -> i32 {
            Summer::sum(
                unsafe { &mut *(ptr as *mut T) },
                ::nola_abi_playground::AbiSafe::into_inner(v),
            )
        }

        &const { ::nola_abi_playground::VTable::new::<T>(DynSummer { sum: __sum::<T> }) }
    }

    pub fn new_box<T: Summer>(
        value: ::std::boxed::Box<T>,
    ) -> ::nola_abi_playground::DynBox<'static, Self> {
        ::nola_abi_playground::DynBox::new(value, Self::vtable::<T>())
    }

    pub fn new_ref<T: Summer>(value: &T) -> ::nola_abi_playground::DynRef<'_, Self> {
        ::nola_abi_playground::DynRef::new(value, Self::vtable::<T>())
    }

    pub fn new_ref_mut<T: Summer>(value: &mut T) -> ::nola_abi_playground::DynRefMut<'_, Self> {
        ::nola_abi_playground::DynRefMut::new(value, Self::vtable::<T>())
    }
}

impl Summer for ::nola_abi_playground::Dyn<'_, DynSummer> {
    fn sum(&mut self, v: ::std::vec::Vec<i32>) -> i32 {
        unsafe {
            (self.vtable().methods().sum)(
                self.data(),
                ::nola_abi_playground::IntoAbiSafe::into_abi_safe(v),
            )
        }
    }
}

#[doc(hidden)]
#[derive(::core::default::Default)]
pub struct __NolaPlaygroundSymbols {
    pub doubled: ::core::sync::atomic::AtomicPtr<()>,
    pub summer1: ::core::sync::atomic::AtomicPtr<()>,
    pub summer2: ::core::sync::atomic::AtomicPtr<()>,
    pub favorite_number: ::core::sync::atomic::AtomicPtr<()>,
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
        unsafe extern "C-unwind" fn __thunk_doubled(
            v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
        ) -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32> {
            unsafe { __LIB1_NolaPlayground::__resolve_doubled()(v) }
        }

        unsafe extern "C-unwind" fn __thunk_summer1()
        -> ::nola_abi_playground::DynBox<'static, DynSummer> {
            unsafe { __LIB1_NolaPlayground::__resolve_summer1()() }
        }

        unsafe extern "C-unwind" fn __thunk_summer2()
        -> ::nola_abi_playground::DynBox<'static, DynSummer> {
            unsafe { __LIB1_NolaPlayground::__resolve_summer2()() }
        }

        unsafe extern "C-unwind" fn __thunk_favorite_number() -> usize {
            unsafe { __LIB1_NolaPlayground::__resolve_favorite_number()() }
        }

        static __SYMBOLS: __NolaPlaygroundSymbols = __NolaPlaygroundSymbols {
            doubled: ::core::sync::atomic::AtomicPtr::new(__thunk_doubled as *mut ()),
            summer1: ::core::sync::atomic::AtomicPtr::new(__thunk_summer1 as *mut ()),
            summer2: ::core::sync::atomic::AtomicPtr::new(__thunk_summer2 as *mut ()),
            favorite_number: ::core::sync::atomic::AtomicPtr::new(
                __thunk_favorite_number as *mut (),
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

    pub fn __resolve_doubled() -> unsafe extern "C-unwind" fn(
        ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
    ) -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32>{
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

    pub fn __resolve_summer1()
    -> unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer> {
        let ptr = Self::__dylib()
            .symbol(c"__nola_0_1_0__get_summer1")
            .as_ptr();
        Self::__symbols()
            .summer1
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(ptr)
        }
    }

    pub fn __resolve_summer2()
    -> unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer> {
        let ptr = Self::__dylib()
            .symbol(c"__nola_0_1_0__get_summer2")
            .as_ptr();
        Self::__symbols()
            .summer2
            .store(ptr, ::core::sync::atomic::Ordering::Relaxed);
        unsafe {
            ::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(ptr)
        }
    }

    pub fn __resolve_favorite_number() -> unsafe extern "C-unwind" fn() -> usize {
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

    pub fn summer1(&self) -> ::nola_abi_playground::DynBox<'_, DynSummer> {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(
                Self::__symbols()
                    .summer1
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))()
        }
    }

    pub fn summer2(&self) -> ::nola_abi_playground::DynBox<'_, DynSummer> {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(
                Self::__symbols()
                    .summer2
                    .load(::core::sync::atomic::Ordering::Relaxed),
            ))()
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
            symbols: ::core::default::Default::default(),
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
    pub fn __resolve_doubled(
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

    pub fn summer1(&self) -> ::nola_abi_playground::DynBox<'_, DynSummer> {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(self.get_or_resolve(&self.symbols.summer1, c"__nola_0_1_0__get_summer1")))(
            )
        }
    }

    #[doc(hidden)]
    pub fn __resolve_summer1(
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

    pub fn summer2(&self) -> ::nola_abi_playground::DynBox<'_, DynSummer> {
        unsafe {
            (::core::mem::transmute::<
                *mut (),
                unsafe extern "C-unwind" fn() -> ::nola_abi_playground::DynBox<'static, DynSummer>,
            >(self.get_or_resolve(&self.symbols.summer2, c"__nola_0_1_0__get_summer2")))(
            )
        }
    }

    #[doc(hidden)]
    pub fn __resolve_summer2(
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
    pub fn __resolve_favorite_number(
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
