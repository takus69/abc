use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        mut n: usize,
    }
    let mut k: Vec<usize> = Vec::new();
    let mut i = 61;
    while n > 0 {
        i -= 1;
        let m = 1 << i;
        if n >= m {
            k.push(i);
            n -= m;
        }
    }
    let mut ans: Vec<usize> = vec![0];
    for i in 1..=k.len() {
        for perm in k.iter().combinations(i) {
            let mut tmp = 0;
            for &&p in perm.iter() {
                tmp += 1 << p;
            }
            ans.push(tmp);
        }
    }
    ans.sort();
    for a in ans.iter() {
        println!("{}", a);
    }
}