use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
        mut pqr: [(usize, usize, usize); t],
    }
    for i in 0..t {
        pqr[i].0 -= 1;
        pqr[i].1 -= 1;
        pqr[i].2 -= 1;
    }
    let mut x: Vec<isize> = vec![0; 20];
    let mut score = 0;
    let mut ans: Vec<char> = Vec::new();
    for &(p, q, r) in pqr.iter() {
        let mut a0 = 0;
        for i in [p, q, r] {
            if x[i] == 0 { a0 -= 1; } else if x[i] == -1 { a0 += 1; }
        }
        let mut b0 = 0;
        for i in [p, q, r] {
            if x[i] == 0 { b0 -= 1; } else if x[i] == 1 { b0 += 1; }
        }
        let d = if a0 > b0 { ans.push('A');1 } else { ans.push('B');-1 };
        for i in [p, q, r] {
            x[i] += d;
        }

        for i in 0..20 {
            if x[i] == 0 { score += 1; }
        }
    }
    eprintln!("score: {}", score);
    println!("{}", ans.iter().join(" "));
}