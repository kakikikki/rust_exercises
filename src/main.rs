use std::time::{Instant};

fn main() {

    for number in (0..51).step_by(10) {
        // Test simple rec
        let start = Instant::now();
        let result = fibonacci(number);
        let duration = start.elapsed();
        println!("Simple recursion: N: {:<3} Result: {} Time: {:?}", number, result, duration);

        // Test fast_fib <3 shadowing
        let start = Instant::now();
        let result = fast_fib(number);
        let duration = start.elapsed();
        println!("Fast doubling:    N: {:<3} Result: {} Time: {:?}", number, result, duration);
    }
}

// Naive approach
fn fibonacci(n: isize) -> isize {

    if n < 2 {
        return n;
    }
    else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

// Fast doubling Rust port
// original algorithm written in python by
// https://www.nayuki.io/page/fast-fibonacci-algorithms
fn fast_fib(n: isize) -> isize {
    return _fast_fib(n).0;
}

fn _fast_fib(n: isize) -> (isize, isize) {

    if n == 0 {
        return (0, 1);
    }
    else {
        let (a, b) = _fast_fib(n / 2);
        let c = a * (b * 2 -a);
        let d = a * a + b * b;

        if n % 2 == 0 {
            return (c, d);
        }
        else {
            return (d, c + d);
        }
    }
}
