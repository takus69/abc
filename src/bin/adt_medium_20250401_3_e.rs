use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut dsu = Dsu::new(n+1);
    for &(a, b) in ab.iter() {
        dsu.merge(a, b);
    }
    let mut cnt: Vec<usize> = vec![0; n+1];
    let mut size: Vec<usize> = vec![0; n+1];
    for &(a, b) in ab.iter() {
        let g = dsu.leader(a);
        cnt[g] += 1;
        size[g] = dsu.size(g);
    }
    let mut ans = 0;
    for i in 1..=n {
        if cnt[i] > 0 && cnt[i] > size[i]-1 {
            ans += cnt[i] - size[i] + 1;
        }
    }
    println!("{}", ans);

}