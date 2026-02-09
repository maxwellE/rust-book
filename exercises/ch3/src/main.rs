fn f_to_c(f: f32) -> f32 {
    return (f - 32.0) * (5.0/9.0)
}

fn c_to_f(c: f32) -> f32 {
    return (c * (9.0/5.0)) + 32.0
}

fn nth_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}

fn main() {
    let f = 72.0;
    let c = f_to_c(f);
    println!("f is {f}, c is {c}");
    let f2 = c_to_f(c);
    println!("c is {c}, f2 is {f2}");
    let n = 10;
    let fib1 = nth_fibonacci(n);
    println!("n is {n}, fibonacci is {fib1}")
}
