use nola_playground_lib::{LIB1, NolaPlayground, Summer};

fn main() {
    println!("=== STATIC MODE (using LIB1) ===\n");

    println!(
        "The library's favorite number is: {}",
        LIB1.favorite_number()
    );

    let mut s1 = LIB1.summer1();
    dbg!(s1.sum(vec![1, 2, 3, 4, 5]));

    let mut s2 = LIB1.summer2();
    dbg!(s2.sum(vec![6, 7, 8, 9, 10]));

    dbg!(LIB1.doubled(vec![1, 2, 3, 4, 5]));

    println!("\n=== INSTANCE MODE (using NolaPlayground::load) ===\n");

    {
        let lib = NolaPlayground::load(
            c"/home/milkey/nola/playground/target/release/deps/libnola_playground_impl.so",
        );

        println!(
            "Instance lib's favorite number is: {}",
            lib.favorite_number()
        );

        let mut s1 = lib.summer1();
        dbg!(s1.sum(vec![10, 20, 30]));

        let mut s2 = lib.summer2();
        dbg!(s2.sum(vec![100, 200, 300]));

        dbg!(lib.doubled(vec![10, 20, 30]));

        println!("\n=== Dropping instance mode lib (dlclose will be called) ===");
    }

    println!("\n=== Static mode still works (no dlclose) ===");
    dbg!(LIB1.doubled(vec![42]));

    println!("\nDone!");
}
