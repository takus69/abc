use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut ans = Vec::new();
    let mut s = HashSet::new();
    for (a, b) in &ab {
        s.insert((a+b-2)%n);
    }
    for _ in 0..(m-s.len()) {
        for k in 0..n {
            if !s.contains(&k) {
                s.insert(k);
                break;
            }
        }
    }

    for si in s {
        for i in 0..n {
            let j = (n + si - i)%n;
            ans.push((i+1, j+1));
        }
    }

    /*
    let mut board = vec![vec![0; n]; n];
    for (i, j) in &ab {
        eprintln!("(i, j): ({}, {})", i, j);
        let mut i2 = i-1;
        let j2 = j-1;
        while board[i2][j2] == 1 {
            i2 += 1;
            i2 %= n;
        }
        for k in 0..n {
            let a = (i2+k)%n+1;
            let b = (j2+k)%n+1;
            board[a-1][b-1] = 1;
            ans.push((a, b));
        }
    }
    */
    // println!("{:?}", board);

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j);
    }
}