use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: String,
    }
    for k in 0..26 {
        let mut tmp = String::new();
        
        for &si in &s {
            let a = si as u8 - b'a';
            let shifted = (a+k)%26 + b'a';
            tmp.push(shifted as char);
        }
        if tmp == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}