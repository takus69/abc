use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    if t > s {
        println!("Yes");
    } else {
        println!("No");
    }
}