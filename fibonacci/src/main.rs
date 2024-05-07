use tailcall::tailcall;

// ignoring potential overflow
fn fibonacci(n: u8) -> usize {
    #[tailcall]
    fn fibonacci_helper(a: usize, b: usize, c: u8) -> usize {
        if c == 0 {
            a
        } else {
            fibonacci_helper(b, a + b, c - 1)
        }
    }
    fibonacci_helper(0, 1, n)
}

fn fibonacci_iter(mut n: u8) -> usize {
    let mut a = 0;
    let mut b = 1;

    while n > 0 {
        n -= 1;
        let tmp = a;
        a = b;
        b = tmp + b;
    }

    a
}

fn main() {
    println!("{}", fibonacci(6));
    println!("{}", fibonacci_iter(6));
}
