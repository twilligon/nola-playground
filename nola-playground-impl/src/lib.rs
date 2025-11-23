#![deny(improper_ctypes)]

use nola_playground_lib::Summer;

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
    fn drop(&mut self) {
        if self.grand_total > 0 {
            dbg!(self.grand_total);
        }
    }
}

fn doubled(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * 2).collect()
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__get_summer1() -> ::nola_playground_lib::__SummerDynBox {
    Summer::__into_dyn_box(Box::new(Summer1))
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__get_summer2() -> ::nola_playground_lib::__SummerDynBox {
    Summer::__into_dyn_box(Box::new(Summer2 { grand_total: 0 }))
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn __nola_0_1_0__doubled(
    v: ::nola_abi_playground::abi_safe::std::vec::Vec<i32>,
) -> ::nola_abi_playground::abi_safe::std::vec::Vec<i32> {
    ::nola_abi_playground::IntoAbiSafe::into_abi_safe(doubled(
        ::nola_abi_playground::AbiSafe::into_inner(v),
    ))
}
