use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut t_c1 = 0;
    let mut c1: Vec<usize> = vec![t_c1];
    let mut t_c2 = 0;
    let mut c2: Vec<usize> = vec![t_c2];
    let mut sep: Vec<usize> = vec![];
    for (i, &si) in s.iter().enumerate() {
        if si == '1' {
            t_c1 += 1;
        } else if si == '2' {
            t_c2 += 1;
        } else {
            sep.push(i);
        }
        c1.push(t_c1);
        c2.push(t_c2);
    }
    let max_c2 = *c2.last().unwrap();
    for i in 0..(n+1) {
        c2[i] = max_c2 - c2[i];
    }
    // println!("c1: {:?}", c1);
    // println!("c2: {:?}", c2);
    // println!("sep: {:?}", sep);
    for _ in 0..q {
        input! {
            mut l: usize,
            mut r: usize,
        }
        l -= 1;

        let mut ok = match sep.binary_search(&l) {
            Ok(index) => index,
            Err(index) => index,
        };
        let mut ng = match sep.binary_search(&r) {
            Ok(index) => index,
            Err(index) => index,
        };
        let mut ans = if ok == ng { 0 } else { 1 };
        // println!("start, ok: {}, ng: {}, ans: {}", ok, ng, ans);
        while ans > 0 && ok < ng {
            let m = (ok + ng) / 2;
            // println!("ok: {}, m: {}, ng: {}", ok, m, ng);
            let i = sep[m];
            // println!("l: {}, i: {}, r: {}", l, i, r);
            let cnt1 = c1[i] - c1[l];
            let cnt2 = c2[i] - c2[r];
            // println!("cnt1: {}, cnt2: {}", cnt1, cnt2);
            ans = ans.max(cnt1.min(cnt2)*2 + 1);
            if cnt1 < cnt2 {
                ok = m+1;
            } else {
                ng = m;
            }
        }
        println!("{}", ans);
    }
}