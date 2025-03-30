use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut c: Vec<(usize, char)> = Vec::new();
    for (i, &ai) in a.iter().enumerate() {
        c.push((ai, 'a'));
    }
    for (i, &bi) in b.iter().enumerate() {
        c.push((bi, 'b'));
    }
    c.sort();
    let mut ans: Vec<usize> = Vec::new();
    for (i, &(_, c)) in c.iter().enumerate() {
        if c == 'a' {
            ans.push(i+1);
        }
    }
    println!("{}", ans.iter().join(" "));

    let mut ans: Vec<usize> = Vec::new();
    for (i, &(_, c)) in c.iter().enumerate() {
        if c == 'b' {
            ans.push(i+1);
        }
    }
    println!("{}", ans.iter().join(" "));
}