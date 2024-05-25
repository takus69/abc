use proconio::input;
use ac_library::{Segtree, Max};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut sg = Segtree::<Max<_>>::new(n);
    for i in 0..n { sg.set(i, a[i]); }
    
    for _ in 0..q {
        input! {
            t: usize,
            l: usize,
            r: usize,
        }
        if t == 1 {
            let x = l-1;
            let v = r as i64;
            sg.set(x, v);
        } else if t == 2 {
            let ans = sg.prod((l-1)..=(r-1));
            println!("{}", ans);
        } else {
            let x = l-1;
            let v = r as i64;
            let f = |&y: &i64| v <= y;
            let ans = sg.max_right(x, f);
            println!("{}", ans+1);
        }
    }
}