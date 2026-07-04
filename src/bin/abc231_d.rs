use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut cnt: Vec<usize> = vec![0; n+1];
    let mut dsu = Dsu::new(n+1);
    for &(a, b) in &ab {
        cnt[a] += 1;
        cnt[b] += 1;
        if cnt[a] > 2 || cnt[b] > 2 || dsu.same(a, b) { println!("No");return; }
        dsu.merge(a, b);
    }
    println!("Yes");
}