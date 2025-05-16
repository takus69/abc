use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        mut rc: [(usize, usize); q],
    }
    let mut gr: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); h];
    let mut gc: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); w];
    for i in 0..h {
        for j in 0..w {
            gr[i].insert(j);
            gc[j].insert(i);
        }
    }
    for i in 0..q {
        let (mut r, mut c) = rc[i];
        r -= 1;
        c -= 1;

        if gr[r].contains(&c) {
            gr[r].remove(&c);
            gc[c].remove(&r);
            continue;
        }

        // 2分探索で(r, c)に一番近い場所を探す
        fn binary_search(i: usize, g: &BTreeSet<usize>) -> (Option<&usize>, Option<&usize>) {
            let l = g.range(..i).next_back();
            let r = g.range(i..).next();
            (l, r)
        }

        let (cl, cr) = binary_search(c, &gr[r]);
        let cl = if let Some(c) = cl { Some(*c) } else { None };
        let cr = if let Some(c) = cr { Some(*c) } else { None };
        if let Some(c) = cl {
            gr[r].remove(&c);
            gc[c].remove(&r);
        }
        if let Some(c) = cr {
            gr[r].remove(&c);
            gc[c].remove(&r);
        }

        let (rl, rr) = binary_search(r, &gc[c]);
        let rl = if let Some(r) = rl { Some(*r) } else { None };
        let rr = if let Some(r) = rr { Some(*r) } else { None };
        if let Some(r) = rl {
            gr[r].remove(&c);
            gc[c].remove(&r);
        }
        if let Some(r) = rr {
            gr[r].remove(&c);
            gc[c].remove(&r);
        }
    }
    
    let mut ans = 0;
    for r in 0..h {
        ans += gr[r].len();
    }
    println!("{}", ans);
}