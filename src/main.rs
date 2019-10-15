use std::time::Instant;

fn main() {
//    struct S;
//    impl S {
//        fn f() { println!("S"); }
//    }
//    trait T1 {
//        fn f() { println!("T1 f"); }
//    }
//    impl T1 for S {}
//    trait T2 {
//        fn f() { println!("T2 f"); }
//    }
//    impl T2 for S {}
//    S::f();  // Calls the inherent impl.
//    <S as T1>::f();  // Calls the T1 trait function.
//    <S as T2>::f();  // Calls the T2 trait function.

    pub fn fibonacci(n: i32) -> i64 {
        match n {
            0 => 0,
            1 | 2 => 1,
            3 => 2,
            _ => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
    let start = Instant::now();
    let result = fibonacci(50);
    let duration = start.elapsed();
    println!("{}", result);
    println!("{:?}", duration);
}
