use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    if s.ends_with("tea") {
        println!("Yes");
    } else {
        println!("No");
    }
}