use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }
    let mut cnt: Vec<(isize, isize)> = vec![(0, 0); m+1];
    for &(a, b) in &ab {
        cnt[a].0 += 1;
        cnt[b].1 += 1;
    }
    for &(c1, c2) in cnt.iter().skip(1) {
        println!("{}", c2-c1);
    }
}