use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = String::from("No");
    let mut j = 0;
    for (i, si) in s.iter().enumerate() {
        if si.to_ascii_uppercase() == t[j] {
            j += 1;
            if j > 2 {
                ans = String::from("Yes");
                break;
            } else if j == 2 && t[j] == 'X' {
                ans = String::from("Yes");
                break;
            }
            continue;
        }
    }
    println!("{}", ans);
}