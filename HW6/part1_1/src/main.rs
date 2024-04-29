use std::time::SystemTime;
//defining the values of k 
fn main() {
fn fib(k: u32) -> u128 {
    match k {
        0 => 0,
        1 => 1,
        _ => fib(k - 2) + fib(k - 1),
    }
}

//finding the runtimes
    for k in 0..=50 {
        let before = SystemTime::now();
        let fk = fib(k);
        let after = SystemTime::now();
        let difference = after.duration_since(before)
            .expect("Time went backwards");
        println!("k: {}, Fk: {}, Time: {:?}", k, fk, difference);
    }
}

