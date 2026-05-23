use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut ans: Vec<char> = Vec::new();
    for si in &s {
        let c = match si[0] {
            'a'|'b'|'c' => { '2' },
            'd'|'e'|'f' => { '3' },
            'g'|'h'|'i' => { '4' },
            'j'|'k'|'l' => { '5' },
            'm'|'n'|'o' => { '6' },
            'p'|'q'|'r'|'s' => { '7' },
            't'|'u'|'v' => { '8' },
            _ => { '9' },
        };
        ans.push(c);
    }

    println!("{}", ans.iter().join(""));
}