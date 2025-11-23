use core::ffi::CStr;
use core::ptr::NonNull;
use libc::{RTLD_LAZY, RTLD_LOCAL, c_void, dlerror, dlopen, dlsym};

pub struct __Dylib {
    handle: NonNull<c_void>,
}

unsafe impl Send for __Dylib {}
unsafe impl Sync for __Dylib {}

impl __Dylib {
    #[inline(never)]
    #[cold]
    #[track_caller]
    fn fail(who: &str, what: impl AsRef<CStr>) -> ! {
        let what = what.as_ref().to_string_lossy();

        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "openbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "solaris",
            target_os = "illumos",
            target_os = "redox",
            target_os = "fuchsia",
            target_os = "cygwin",
        ))]
        if let Some(err) = NonNull::new(unsafe { dlerror() }) {
            let why = unsafe { CStr::from_ptr(err.as_ptr()) }.to_string_lossy();

            panic!("{who} failed: {what}: {why}");
        }

        panic!("{who} failed: {what}");
    }

    #[inline]
    pub fn open(path: impl AsRef<CStr>) -> Self {
        let ptr = unsafe { dlopen(path.as_ref().as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
        let handle = NonNull::new(ptr).unwrap_or_else(|| Self::fail("dlopen", path));

        Self { handle }
    }

    #[inline]
    pub fn symbol(&self, name: impl AsRef<CStr>) -> NonNull<c_void> {
        let ptr = unsafe { dlsym(self.handle.as_ptr(), name.as_ref().as_ptr()) };
        NonNull::new(ptr).unwrap_or_else(|| Self::fail("dlsym", name))
    }
}

// why not impl Drop for __Dylib? see https://internals.rust-lang.org/t/15169
