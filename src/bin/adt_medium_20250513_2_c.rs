use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }
    if s.len() != 8 {
        println!("No");
        return;
    }
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    if chars.contains(&s[0]) && chars.contains(&s[s.len()-1]) {
        let ss = s[1..(s.len()-1)].iter().join("");
        match ss.parse::<usize>() {
            Ok(a) => {
                if 100000 <= a && a <= 999999 {
                    println!("Yes");
                    return;
                }
            },
            Err(_) => {},
        }
    }
    println!("No");
}