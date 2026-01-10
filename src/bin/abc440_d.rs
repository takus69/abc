use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        xy: [(usize, usize); q],
    }
    a.sort();
    for &(x, y) in &xy {
        let l = match a.binary_search(&x) {
            Ok(i) => { i },
            Err(i) => { i },
        };
        let mut ng: usize = x-1;
        let mut ok: usize = 2_000_000_000;
        while ng+1 < ok {
            let m = (ng + ok) / 2;
            let cnt = match a.binary_search(&m) {
                Ok(i) => { m-(i-l+1) },
                Err(i) => { m-(i-l) },
            };
            if cnt < y+x-1 {
                ng = m;
            } else {
                ok = m;
            }
        }
        println!("{}", ok);

    }
}