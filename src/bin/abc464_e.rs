use proconio::input;
use itertools::Itertools;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rcx: [(usize, usize, char); q],
    }
    let mut max: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for (i, &(r, c, x)) in rcx.iter().enumerate() {
        max[r-1][c-1] = i+1;
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let mut o = max[i][j];
            if i < h-1 {
                o = o.max(max[i+1][j]);
            }
            if j < w-1 {
                o = o.max(max[i][j+1]);
            }
            max[i][j] = o;
        }
    }

    let mut ans: Vec<Vec<char>> = vec![vec!['A'; w]; h];
    for i in 0..h {
        for j in 0..w {
            let (_, _, x) = if max[i][j]==0 { (0, 0, 'A') } else { rcx[max[i][j]-1] };
            ans[i][j] = x;
        }
    }

    for a in &ans {
        println!("{}", a.iter().join(""));
    }
}