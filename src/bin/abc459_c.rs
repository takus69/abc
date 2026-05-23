use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: isize,
        q: usize,
    }
    const INF: usize = 1000000;
    let mut ft = FenwickTree::new(INF, 0isize);
    let mut idx: Vec<usize> = vec![0; (n+1) as usize];
    let mut base = 0;
    ft.add(0, n);
    for _ in 0..q {
        input! {
            c: usize,
            x: usize,
        }
        if c == 1 {
            let i = idx[x];
            ft.add(i, -1);
            ft.add(i+1, 1);
            idx[x] += 1;
            if ft.sum(base..(base+1)) == 0 {
                base += 1;
            }
        } else {
            let ans = ft.sum((x+base)..INF);
            println!("{}", ans);
        }
    }
}