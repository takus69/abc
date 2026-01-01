use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }
    let mut ans: Vec<usize> = vec![24; d];
    for &(li, ri, hi) in lrh.iter() {
        for i in (li-1)..ri {
            ans[i] = ans[i].min(hi);
        }
    }
    println!("{}", ans.iter().sum::<usize>());
}