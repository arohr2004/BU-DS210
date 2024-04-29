// Q2
fn main() {
    // Part (a): Using u128
    let mut fib_nums: [u128; 101] = [0; 101];
    fib_nums[1] = 1;

    for i in 2..=100 {
        fib_nums[i] = fib_nums[i - 1] + fib_nums[i - 2];
    }

    println!("Fibonacci numbers using u128:");
    for (i, &fib) in fib_nums.iter().enumerate() {
        println!("F[{}]: {}", i, fib);
    }

    // Part (b): Using u8
    let mut fib_nums_u8: [u8; 101] = [0; 101];
    fib_nums_u8[1] = 1;

    for i in 2..=100 {
        fib_nums_u8[i] = fib_nums_u8[i - 1].wrapping_add(fib_nums_u8[i - 2]);
    }

    println!("Fibonacci numbers using u8:");
    for (i, &fib) in fib_nums_u8.iter().enumerate() {
        println!("F[{}]: {}", i, fib);
    }
}

