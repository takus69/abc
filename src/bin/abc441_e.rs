use proconio::{input, marker::Chars};
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut a_cnt: usize = 0;
    let mut b_cnt: usize = 0;
    let mut a_cumsum: Vec<usize> = Vec::new();
    let mut b_cumsum: Vec<usize> = Vec::new();
    for &si in s.iter() {
        if si == 'A' {
            a_cnt += 1;
        } else if si == 'B' {
            b_cnt += 1;
        }
        a_cumsum.push(a_cnt);
        b_cumsum.push(b_cnt);
    }
    let base = 1_000_000;  // 0相当
    let mut fw: FenwickTree<isize> = FenwickTree::new(base*2, 0);
    for i in 0..n {
        fw.add(base+a_cumsum[i]-b_cumsum[i], 1);
    }
    
    let mut ans = fw.sum((base+1)..base*2);
    for i in 0..n {
        let d = base+a_cumsum[i]-b_cumsum[i];
        fw.add(d, -1);
        ans += fw.sum((d+1)..base*2);
    }
    println!("{}", ans);
}