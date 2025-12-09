use std::usize;

fn main() {
    for i in 1..=10 {
        println!("fibo({i}) = {}", fibo(i));
    }
}

fn fibo(n: usize) -> usize {
    if n <= 2 {
        return 1;
    }
    fibo(n - 1) + fibo(n - 2)
}