use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        p: [usize; n],
        q: usize,
        lr: [(i64, i64); q],
    }
    let mut cumsum: Vec<usize> = Vec::new();
    let mut sum_p = 0;
    for &pi in p.iter() {
        sum_p += pi;
        cumsum.push(sum_p);
    }
    for (l, r) in lr.iter() {
        let li = match x.binary_search(l) {
            Ok(i) => { if i == 0 { usize::MAX } else { i-1 } },
            Err(i) => { if i == 0 { usize::MAX } else { i-1 } },
        };
        let ri = match x.binary_search(r) {
            Ok(i) => { i },
            Err(i) => { if i == 0 { usize::MAX } else { i-1 } },
        };
        let ans = if ri == usize::MAX {
            0
        } else if li == usize::MAX {
            cumsum[ri]
        } else {
            cumsum[ri] - cumsum[li]
        };

        println!("{}", ans);
    }
}