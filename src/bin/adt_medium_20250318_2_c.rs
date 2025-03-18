use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut s = 0;
    let mut t = 0;
    let mut i = 1;
    let mut ans: Vec<usize> = Vec::new();

    while t < m {
        while i < a[s] {
            ans.push(i);
            i += 1;
        }
        while t < m-1 && a[t]+1 == a[t+1] {
            t += 1;
        }
        for i in (a[s]..=(a[t]+1)).rev() {
            ans.push(i);
        }
        i = a[t]+2;
        t += 1;
        s = t;
    }

    let mut ans_max = *ans.iter().max().unwrap_or(&0); 
    while ans.len() < n {
        ans_max += 1;
        ans.push(ans_max);
    }

    println!("{}", ans.iter().join(" "));
}