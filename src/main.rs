mod fibonaacci;

fn main() {
    let ans = is_even(9);
    println!("{}", ans);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}