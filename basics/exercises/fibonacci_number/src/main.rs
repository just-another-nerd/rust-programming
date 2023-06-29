
fn fib(n: i32) -> i32 {

    if n <= 1 { return n; }
    return fib(n - 1) + fib(n - 2)
}

fn main() {
    let x = 9;
    println!("The {}th fibonacci number is {}", x, fib(x));
}
