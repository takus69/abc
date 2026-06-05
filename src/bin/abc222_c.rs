use proconio::{input, marker::Chars};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n],
    }
    fn gcp(a: char, b: char) -> bool {
        (a=='G' && b=='C') || (a=='C' && b=='P') || (a=='P' && b=='G')
    }
    let mut data: Vec<(usize, Reverse<usize>)> = (0..n*2).map(|x| (0, Reverse(x))).collect();
    let mut rnk: BinaryHeap<(usize, Reverse<usize>)> = BinaryHeap::from(data);
    for i in 0..m {  // ラウンド
        data = Vec::new();
        for _ in 0..n {
            let (mut c1, Reverse(j1)) = rnk.pop().unwrap();
            let (mut c2, Reverse(j2)) = rnk.pop().unwrap();
            if gcp(a[j1][i], a[j2][i]) { c1 += 1; }
            if gcp(a[j2][i], a[j1][i]) { c2 += 1; }
            data.push((c1, Reverse(j1)));
            data.push((c2, Reverse(j2)));
        }
        rnk = BinaryHeap::from(data);
    }
    while let Some((_, Reverse(j))) = rnk.pop() {
        println!("{}", j+1);
    }

}