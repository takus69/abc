use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = Dsu::new(n);

    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }

        if t == 0 {
            dsu.merge(u, v);
            continue;
        }
        if dsu.same(u, v) {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
    }
}