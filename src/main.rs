use nola_playground_lib::{Summer, nola_playground_impl};

fn main() {
    let mut s1 = nola_playground_impl::summer1();
    dbg!(s1.sum(vec![1, 2, 3, 4, 5]));

    let mut s2 = nola_playground_impl::summer2();
    dbg!(s2.sum(vec![6, 7, 8, 9, 10]));

    dbg!(nola_playground_impl::doubled(vec![1, 2, 3, 4, 5]));
}
