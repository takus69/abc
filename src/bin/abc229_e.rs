use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in &ab {
        edge[a.min(b)].push(a.max(b));
    }
    let mut ans: Vec<usize> = vec![0];
    let mut cnt: usize = 0;
    let mut dsu = Dsu::new(n+1);
    for i in (1..=n).rev() {
        cnt += 1;
        for &j in edge[i].iter() {
            if dsu.same(i, j) {
                continue;
            }
            dsu.merge(i, j);
            cnt -= 1;
        }
        ans.push(cnt);
    }
    ans.reverse();
    for a in ans.iter().skip(1) {
        println!("{}", a);
    }
}