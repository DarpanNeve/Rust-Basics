mod fibonaacci;
use fibonaacci::fib;
fn main() {
    let ans = is_even(9);
    println!("{}", ans);
    println!("The fibonacci series value is {}", fib(10))
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}