use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, usize); m],
    }
    abc.sort_by(|a, b| a.2.cmp(&b.2));
    let mut dsu: Dsu = Dsu::new(n+1);
    let mut ans = 0;
    for &(a, b, c) in abc.iter() {
        if dsu.same(a, b) { continue; }
        ans += c;
        dsu.merge(a, b);
    }

    println!("{}", ans);
}