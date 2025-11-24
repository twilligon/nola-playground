use core::marker::PhantomData;
use core::mem::{align_of, needs_drop, size_of};
use core::ops::{Deref, DerefMut, Drop};
use std::alloc::{Layout, dealloc};

#[repr(C)]
pub struct VTable<V> {
    drop_in_place: Option<unsafe extern "C-unwind" fn(*mut ())>,
    size: usize,
    align: usize,
    methods: V,
}

impl<V> VTable<V> {
    pub const fn new<T>(methods: V) -> Self {
        Self {
            drop_in_place: if needs_drop::<T>() {
                Some(crate::__drop_in_place::<T>)
            } else {
                None
            },
            size: size_of::<T>(),
            align: align_of::<T>(),
            methods,
        }
    }

    pub fn methods(&self) -> &V {
        &self.methods
    }
}

#[repr(C)]
pub struct Dyn<'lib, V> {
    data: *mut (),
    vtable: *const VTable<V>,
    phantom: PhantomData<&'lib ()>,
}

unsafe impl<V: Sync> Sync for Dyn<'_, V> {}
unsafe impl<V: Send> Send for Dyn<'_, V> {}

#[repr(transparent)]
pub struct DynBox<'lib, V>(Dyn<'lib, V>);

unsafe impl<V: Sync> Sync for DynBox<'_, V> {}
unsafe impl<V: Send> Send for DynBox<'_, V> {}

impl<V> DynBox<'_, V> {
    pub fn new<T>(value: Box<T>, vtable: &'static VTable<V>) -> Self {
        Self(Dyn {
            data: Box::into_raw(value) as *mut (),
            vtable,
            phantom: PhantomData,
        })
    }
}

impl<'lib, V> Deref for DynBox<'lib, V> {
    type Target = Dyn<'lib, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for DynBox<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> Drop for DynBox<'_, V> {
    fn drop(&mut self) {
        unsafe {
            let vtable = &*self.0.vtable;
            if let Some(drop_fn) = vtable.drop_in_place {
                drop_fn(self.0.data);
            }
            if vtable.size > 0 {
                dealloc(
                    self.0.data as *mut u8,
                    Layout::from_size_align_unchecked(vtable.size, vtable.align),
                );
            }
        }
    }
}

#[repr(transparent)]
pub struct DynRef<'a, V>(Dyn<'a, V>);

unsafe impl<V: Sync> Sync for DynRef<'_, V> {}
unsafe impl<V: Send> Send for DynRef<'_, V> {}

impl<'a, V> DynRef<'a, V> {
    pub fn new<T>(value: &'a T, vtable: &'static VTable<V>) -> Self {
        Self(Dyn {
            data: value as *const T as *mut (),
            vtable,
            phantom: PhantomData,
        })
    }
}

impl<'a, V> Deref for DynRef<'a, V> {
    type Target = Dyn<'a, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct DynRefMut<'a, V>(Dyn<'a, V>);

unsafe impl<V: Sync> Sync for DynRefMut<'_, V> {}
unsafe impl<V: Send> Send for DynRefMut<'_, V> {}

impl<'a, V> DynRefMut<'a, V> {
    pub fn new<T>(value: &'a mut T, vtable: &'static VTable<V>) -> Self {
        Self(Dyn {
            data: value as *mut T as *mut (),
            vtable,
            phantom: PhantomData,
        })
    }
}

impl<'a, V> Deref for DynRefMut<'a, V> {
    type Target = Dyn<'a, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for DynRefMut<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'lib, V> Dyn<'lib, V> {
    pub fn data(&self) -> *mut () {
        self.data
    }

    pub fn vtable(&self) -> &VTable<V> {
        unsafe { &*self.vtable }
    }
}
