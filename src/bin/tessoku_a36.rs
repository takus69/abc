use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    if 2*(n-1) <= k && k%2==0 {
        println!("Yes");
    } else {
        println!("No");
    }
}