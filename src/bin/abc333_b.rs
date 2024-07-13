use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let vertex = vec!['A', 'B', 'C', 'D', 'E'];
    let i1 = vertex.iter().position(|&x| x == s[0]).unwrap();
    let i2 = vertex.iter().position(|&x| x == s[1]).unwrap();
    let j1 = vertex.iter().position(|&x| x == t[0]).unwrap();
    let j2 = vertex.iter().position(|&x| x == t[1]).unwrap();
    let mut sl = i1.max(i2) - i1.min(i2);
    let mut tl = j1.max(j2) - j1.min(j2);
    if sl > 2 { sl = 5 - sl; }
    if tl > 2 { tl = 5 - tl; }
    if sl == tl {
        println!("Yes");
    } else {
        println!("No");
    }
}