use core::ffi::CStr;
use core::ptr::NonNull;
use libc::{RTLD_LAZY, RTLD_LOCAL, c_void, dlclose, dlerror, dlopen, dlsym};

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
        // only dlerror on platforms known to have MT-safe dlerror
        // (i stole this from libloading lol)
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
            // `why` usually contains `what`
            //panic!("{who} failed: {what}: {why}");
            panic!("{who} failed: {why}");
        }

        let what = what.as_ref().to_string_lossy();
        panic!("{who} failed: {what}");
    }

    pub fn open(path: impl AsRef<CStr>) -> Self {
        let ptr = unsafe { dlopen(path.as_ref().as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
        let handle = NonNull::new(ptr).unwrap_or_else(|| Self::fail("dlopen", path));
        Self { handle }
    }

    pub fn symbol(&self, name: impl AsRef<CStr>) -> NonNull<()> {
        self.try_symbol(name.as_ref())
            .unwrap_or_else(|| Self::fail("dlsym", name))
    }

    pub fn try_symbol(&self, name: impl AsRef<CStr>) -> Option<NonNull<()>> {
        let ptr = unsafe { dlsym(self.handle.as_ptr(), name.as_ref().as_ptr()) };
        NonNull::new(ptr.cast())
    }
}

impl Drop for __Dylib {
    fn drop(&mut self) {
        unsafe { dlclose(self.handle.as_ptr()) };
    }
}
