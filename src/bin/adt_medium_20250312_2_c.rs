use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    if s.len() < t.len() {
        println!("No");
        std::process::exit(0);
    }
    for i in 0..=(s.len()-t.len()) {
        if s[i..(i+t.len())] == t {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}