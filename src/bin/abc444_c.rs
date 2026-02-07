use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    // println!("a: {:?}", a);
    let mut ans: Vec<usize> = Vec::new();
    // min+maxと同じか
    if n%2==0 {
        let mut cand = a[0] + a[n-1];
        for i in 1..(n/2) {
            // println!("cand: {}, +: {}", cand, a[i]+a[n-i-1]);
            if a[i] + a[n-i-1] != cand {
                cand = usize::MAX;
                break;
            }
        }
        if cand != usize::MAX {
            ans.push(cand);
        }
    }

    // maxと同じ
    let mut cand = a[n-1];
    while let Some(&tmp) = a.last() {
        if tmp == cand {
            a.pop();
        } else {
            break;
        }
    }
    let n = a.len();
    if n%2==0 {
        for i in 1..(n/2) {
            if a[i] + a[n-i-1] != cand {
                cand = usize::MAX;
                break;
            }
        }
        if cand != usize::MAX {
            ans.push(cand);
        }
    }

    ans.sort();
    println!("{}", ans.iter().join(" "));
}