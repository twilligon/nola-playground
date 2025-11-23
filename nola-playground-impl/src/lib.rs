#![deny(improper_ctypes)]

use std::{mem, ptr};

use nola_playground_lib::nola_playground_impl::{__SummerDynBox, __SummerVtable};
use nola_playground_lib::{Summer, VecAbi, abi_to_vec, vec_to_abi};

struct Summer1;

impl Summer for Summer1 {
    fn sum(&mut self, v: Vec<i32>) -> i32 {
        v.into_iter().sum()
    }
}

struct Summer2 {
    grand_total: i32,
}

impl Summer for Summer2 {
    fn sum(&mut self, v: Vec<i32>) -> i32 {
        let sum: i32 = v.into_iter().sum();
        self.grand_total += sum;
        sum
    }
}

impl Drop for Summer2 {
    #[inline(never)]
    fn drop(&mut self) {
        if self.grand_total > 0 {
            dbg!(self.grand_total);
        }
    }
}

fn doubled(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * 2).collect()
}

#[allow(non_snake_case)]
unsafe extern "C-unwind" fn __Summer_sum<T: Summer>(ptr: *mut (), v: VecAbi<i32>) -> i32 {
    unsafe { &mut *(ptr as *mut T) }.sum(abi_to_vec(v))
}

#[allow(non_snake_case)]
unsafe extern "C-unwind" fn __Drop_drop<T>(ptr: *mut ()) {
    unsafe {
        ptr::drop_in_place(ptr as *mut T);
    }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__get_summer1() -> __SummerDynBox {
    __SummerDynBox {
        data: Box::into_raw(Box::new(Summer1)) as *mut (),
        vtable: &const {
            __SummerVtable {
                drop: if mem::needs_drop::<Summer1>() {
                    Some(__Drop_drop::<Summer1>)
                } else {
                    None
                },
                size: mem::size_of::<Summer1>(),
                align: mem::align_of::<Summer1>(),
                __Summer_sum: __Summer_sum::<Summer1>,
            }
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__get_summer2() -> __SummerDynBox {
    __SummerDynBox {
        data: Box::into_raw(Box::new(Summer2 { grand_total: 0 })) as *mut (),
        vtable: &const {
            __SummerVtable {
                drop: if mem::needs_drop::<Summer2>() {
                    Some(__Drop_drop::<Summer2>)
                } else {
                    None
                },
                size: mem::size_of::<Summer2>(),
                align: mem::align_of::<Summer2>(),
                __Summer_sum: __Summer_sum::<Summer2>,
            }
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__doubled(v: VecAbi<i32>) -> VecAbi<i32> {
    vec_to_abi(doubled(abi_to_vec(v)))
}
