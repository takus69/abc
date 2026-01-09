use proconio::input;
use ac_library::{Segtree, Max};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = Segtree::<Max<usize>>::new(n+1);
    for i in 1..=n {
        seg.set(i, 0);
    }
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                seg.set(p, x);
            },
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", seg.prod(l..r));
            },
            _ => {},
        }
    }
}