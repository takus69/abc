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
    let mut visited = vec![false; n];
    let mut end = 0;
    let mut end_flg = false;
    for i in 0..n {
        if visited[i] { continue; } else { visited[i] = true; }
        let mut ai = i;
        loop {
            ai = a[ai];
            if visited[ai] {
                end = ai;
                break;
            } else {
                visited[ai] = true;
            }
        }
        break;
    }
    let mut ai = end;
    let mut ans = vec![ai+1];
    loop {
        if end == a[ai] {
            break;
        } else {
            ai = a[ai];
            ans.push(ai+1)
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}