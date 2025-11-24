# nola-playground

## The problem: Rust has no stable ABI

Rust does not have a stable ABI (except when using `#[repr(C)]`). If you pass arbitrary types across an ABI boundary—for example, to or from a dynamically-loaded plugin—they can have *different representations on each side*.

rustc reserves the right to change type layouts for reasons as arbitrary as changed flags, profile, version, build configuration, or [running out of gas](https://github.com/rust-lang/rust/pull/40377). So your plugin might silently reinterpret a `Vec<i32>`'s `ptr`, `len`, `cap` as `len`, `ptr`, `cap`. Suddenly your `Vec` has trillions of entries stored at `0x0000000000000005`. Needless to say, this is Undefined Behavior.

## The solution: convert at the boundary

If at the ABI boundary we convert every `Vec` to and from a `#[repr(C)] struct VecAbi` that *just so happens* to exactly match `Vec`'s representation *on most machines*, there are two possibilities:

- **If we're lucky** (i.e. your rustc lays out `Vec` same as mine), conversions are optimized into nothing but a memcpy, then eliminated entirely by standard LLVM optimizations. ABI-safe wrappers for functions taking or returning `Vec` are optimized into simple calls to the underlying function, then inlined, then optimized away entirely by identical code folding when the linker sees the wrapper's machine code is byte-for-byte identical to the underlying function.

- **If we're unlucky**, we have to move a couple integers around so `Vec`'s arbitrary representation matches `VecAbi`'s defined representation. We pay one function call, a few register swaps, and a few bytes in code size.

So like the "yolo" approach (just passing types directly), we bet on "nice coincidences" which work the vast majority of the time. But unlike the yolo approach, **the only cost to being wrong is (very slight!) performance rather than correctness**.

The end goal is a crate (call it `nola-abi`) that will provide:

- **Traits** for converting between Rust types and ABI-safe representations:
  ```rust
  pub unsafe trait AbiRefSafe {}

  pub unsafe trait AbiSafe<T>: AbiRefSafe {
      fn into_inner(self) -> T;
  }

  pub trait IntoAbiSafe: Sized {
      type AbiRepr: AbiSafe<Self>;
      fn into_abi_safe(self) -> Self::AbiRepr;
  }
  ```

- **ABI-safe representations** of standard library types: `__VecAbi`, `__BoxAbi`, `__StringAbi`, `__OptionAbi` (though actually in some cases---frustratingly, only some---`Option` is already ABI-safe), `__SliceAbi`, `__RefAbi`, etc.

- **Derive macros** like `#[derive(AbiRefSafe, IntoAbiSafe)]` and attribute macros like `#[abi_export]`, `#[abi_import]`, `#[abi_trait]`.

But we're not there yet. We need to prototype all this in `nola-playground` first.

## What is nola-playground?

nola-playground is a hand-written prototype of what nola-abi's proc macros will eventually generate. Rather than jumping straight into macro development, we're first writing out the expanded code by hand to iterate on the patterns, discover edge cases, and refine the design before codifying anything.

The code here is intentionally verbose and explicit. It represents the fully-expanded output of hypothetical `#[nola::export]` and `#[nola::import]` attributes, written as if a macro had generated it.

### Why hand-write first?

Proc macros are notoriously difficult to debug and iterate on. By first writing the target output by hand, we can:

1. Experiment with different patterns quickly
2. Ensure the generated code compiles and works correctly
3. Refine the ABI-safe type representations
4. Work out the symbol resolution and dynamic loading strategy
5. Validate that the approach is sound before investing in macro infrastructure

Once we're confident in the patterns here, translating them into proc macros becomes a mechanical exercise.

## The hygiene constraint

A critical requirement is that the generated code must be **hygienic**. In the context of proc macros, this means the generated code cannot rely on any `use` statements being present in the user's code. Every path must be fully qualified from the crate root.

For example, instead of:
```rust
use std::sync::atomic::AtomicPtr;
let ptr = AtomicPtr::new(...);
```

We write:
```rust
let ptr = ::core::sync::atomic::AtomicPtr::new(...);
```

This ensures the macro output works regardless of what the user has imported, shadowed, or renamed. For much of nola-playground-lib, you'll see everything spelled out with `::core::...`, `::std::...`, and `::nola_abi_playground::...` prefixes, a sign this code would in nola-abi et al be output by a macro.

## Workspace structure

### nola-abi-playground

This crate provides the ABI infrastructure that nola-abi will eventually provide. It's separate from nola-abi itself so we can iterate without touching the real crate.

It contains:

- **Traits**: `AbiRefSafe`, `AbiSafe<T>`, and `IntoAbiSafe` form the trait hierarchy for converting between Rust types and their ABI-safe representations.

- **ABI-safe type representations**: Under `abi_safe::std::*`, mirroring the standard library's module structure. For example, `nola_abi_playground::abi_safe::std::vec::Vec<T>` is the `#[repr(C)]` equivalent of `std::vec::Vec<T>`. The module path intentionally mirrors std so the correspondence is obvious.

- **Dynamic library loading**: The `__Dylib` type encapsulates `dlopen`/`dlsym` (on Unix) so that generated code doesn't need to interact with libc directly.

### nola-playground-lib

This is the "generated" code—what a proc macro would emit. It demonstrates:

- **Lazy symbol resolution**: Each imported function has a static `AtomicPtr` that starts pointing to a thunk. On first call, the thunk resolves the real symbol via `dlsym`, atomically updates the pointer, and tail-calls the real function. Subsequent calls go directly to the resolved symbol.

- **ABI conversions**: Arguments are converted to ABI-safe representations before crossing the FFI boundary, and return values are converted back. This uses the `IntoAbiSafe` and `AbiSafe` traits with fully-qualified paths.

- **Module-level `__DYLIB`**: A `LazyLock<__Dylib>` that opens the plugin library once, shared by all imported functions in that module.

### nola-playground-impl

The "plugin" side—a cdylib that exports functions with mangled names (e.g., `__nola_0_1_0__doubled`). This is what a library author would write, with `#[nola::export]` annotations that would generate the `#[unsafe(no_mangle)] extern "C-unwind"` wrappers.

### nola-playground

A binary that exercises everything, proving the pieces fit together.
