fn main() {
    let c: i64 = 30;
    let f: i64 = 80;
    println!("Convert {} C to {} F.", c, convert_to_f(c));
    println!("Convert {} F to {} C.", f, convert_to_c(f));
    println!("Let's get the 5th fib number: {}", fib(20));
}

fn convert_to_f(c: i64) -> i64 {
    (c * 9 / 5) + 32
}

fn convert_to_c(f: i64) -> i64 {
    (f - 32) * 5 / 9
}

fn fib(n: i64) -> i64 {
    // We can do this the normal loopy way.
    // This is so gross to store the whole thing...
    let mut fib_sequence: Vec<i64> = vec![1, 1];

    for i in 0..n + 1 {
        if i == 0 || i == 1 {
            continue;
        }
        fib_sequence.push(fib_sequence[(i - 2) as usize] + fib_sequence[(i - 1) as usize]);
    }
    fib_sequence[(n - 1) as usize]
}
