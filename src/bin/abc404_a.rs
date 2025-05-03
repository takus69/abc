use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let c: Vec<char> = "abcdefgeijklmnopqrstuvwxyz".chars().collect();
    for ci in c.iter() {
        if !s.contains(ci) {
            println!("{}", ci);
            break;
        }
    }
}