use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut ft = FenwickTree::new(n, 0);
    for (i, ai) in a.iter().enumerate() {
        ft.add(i, ai);
    }

    for _ in 0..q {
        input! {
            t: usize,
            l: usize,
            r: usize,
        }
        // println!("q: {} {} {}", t, l, r);

        if t == 0 {
            let i = l;
            let x = r;
            ft.add(i, x);
        } else {
            let ans = ft.sum(l..r);
            println!("{}", ans);
        }
    }
}