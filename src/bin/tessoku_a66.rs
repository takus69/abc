use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut dsu: Dsu = Dsu::new(n+1);
    for _ in 0..q {
        input! {
            c: usize,
            u: usize,
            v: usize,
        }
        match c {
            1 => {
                dsu.merge(u, v);
            },
            2 => {
                if dsu.same(u, v) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => {},
        }
    }
}
