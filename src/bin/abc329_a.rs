use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}