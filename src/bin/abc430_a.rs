use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    if c >= a && d < b {
        println!("Yes");
    } else {
        println!("No");
    }
}