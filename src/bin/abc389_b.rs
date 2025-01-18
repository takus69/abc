use proconio::input;

fn main() {
    input! {
        mut x: usize,
    }
    for i in 1..=21 {
        x /= i;
        if x == 1 {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}