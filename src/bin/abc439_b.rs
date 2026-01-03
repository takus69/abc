use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    }
    while n.len() > 1 {
        let mut tmp = 0;
        for &c in n.iter() {
            let d = c.to_digit(10).unwrap();
            tmp += d*d;
        }
        n = tmp.to_string().chars().collect();
    }

    if n[0] == '1' {
        println!("Yes");
    } else {
        println!("No");
    }
}