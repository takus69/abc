use proconio::{input, marker::Chars};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut r: isize,
        mut c: isize,
        s: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let (mut sr, mut sc) = (2*n, 2*n);
    let r = (r + 2*n as isize) as usize;
    let c = (c + 2*n as isize) as usize;
    set.insert((sr, sc));
    for &si in s.iter() {
        match si {
            'N' => { sr -= 1; },
            'S' => { sr += 1; },
            'E' => { sc += 1; },
            'W' => { sc -= 1; },
            _ => {},
        }
        set.insert((sr, sc));

        let dr = sr + 2*n;
        let dc = sc + 2*n;
        let r2 = dr - r;
        let c2 = dc - c;
        if set.contains(&(r2, c2)) {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }
    println!("{}", ans.iter().join(""));
}