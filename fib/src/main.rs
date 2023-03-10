fn main() {
    println!("{}", compute_fib(1));
    println!("{}", compute_fib(2));
    println!("{}", compute_fib(5));
    println!("{}", compute_fib(6));
}

fn compute_fib(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => compute_fib(x - 1) + compute_fib(x - 2),
    }
}

