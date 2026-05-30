use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, isize); m],
    }
    abc.sort_by_key(|x| x.2);
    let mut dsu = Dsu::new(n+1);
    let mut ans = 0;
    for &(a, b, c) in &abc {
        if dsu.same(a, b) && c > 0 {
            ans += c;
        } else {
            dsu.merge(a, b);
        }
    }
    println!("{}", ans);
}