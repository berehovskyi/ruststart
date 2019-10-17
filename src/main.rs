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
    pub fn count_to(n :i64) -> i64 {
        let mut sum :i64 = 0;
        for i in 0..n {
            sum += i * i + 2 * i;
        }
        return sum
    }
    let start = Instant::now();
//    let result = fibonacci(50);
    let result = count_to(1_000_000_000);
    let duration = start.elapsed();
    println!("{}", result < std::i32::MAX as i64);
    println!("{}", result);
    println!("{:?}", duration);
}
