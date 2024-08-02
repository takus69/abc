use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a%3 != 0 && a+1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}