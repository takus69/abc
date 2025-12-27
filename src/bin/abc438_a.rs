use proconio::input;

fn main() {
    input! {
        d: usize,
        f: usize,
    }
    println!("{}", (7-(d-f)%7));
}