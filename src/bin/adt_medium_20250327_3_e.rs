use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut j = 0;
    for (i, &ti) in t.iter().enumerate() {
        if i == 2 && ti == 'X' {
            println!("Yes");
            return;
        }
        while s[j] != ti.to_ascii_lowercase() {
            j += 1;
            if j == s.len() {
                println!("No");
                return;
            }
        }
        j += 1;
    }
    println!("Yes");
}