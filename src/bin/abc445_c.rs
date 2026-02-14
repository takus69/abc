use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    for i in 0..n {
        a[i] -= 1;
    }

    let mut done: Vec<bool> = vec![false; n];
    let mut ans: Vec<usize> = vec![usize::MAX; n];
    for i in 0..n {
        if done[i] { continue; }
        let mut targets: Vec<usize> = vec![i];
        let mut now = i;
        loop {
            let next = a[now];
            if now == next { break; }
            targets.push(next);
            now = next;

        }
        for &j in targets.iter() {
            done[j] = true;
            ans[j] = now+1;
        }
    }

    println!("{}", ans.iter().join(" "));
}