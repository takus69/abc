use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }
    if t.max(a) > n/2 {
        println!("Yes");
    } else {
        println!("No");
    }
}