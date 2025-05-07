use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut i = 0;
    for &si in s.iter() {
        if si.to_ascii_uppercase() == t[i] || (i == 2 && t[i]=='X') {
            i += 1;
        }
        if i > 2 {
            break;
        }
    }
    if i > 2 || (i == 2 && t[2] == 'X') {
        println!("Yes");
    } else {
        println!("No");
    }
}