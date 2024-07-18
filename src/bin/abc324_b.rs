use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    if n == 1 {
        println!("Yes");
        std::process::exit(0);
    }
    while n%2 == 0 || n%3 == 0 {
        if n%2 == 0 { n /= 2; }
        if n%3 == 0 { n /= 3; }
        if n == 1 {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}