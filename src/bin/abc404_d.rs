use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; n],
    }
    let mut v: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for i in 1..=m {
        input! {
            k: usize,
            a: [usize; k],
        }
        for &ai in a.iter() {
            v[ai].push(i);
        }
    }
    let mut ans = usize::MAX;
    for i in 0..3_usize.pow(n as u32) {
        let mut perm: Vec<usize> = Vec::new();
        let mut i = i;
        for j in 0..n {
            perm.push(i%3);
            i /= 3;
        }

        let mut tmp = 0;
        let mut vv = vec![0; m+1];
        for (i, &pi) in perm.iter().enumerate() {
            let ni = i+1;
            tmp += pi * c[i];
            for &vi in v[ni].iter() {
                vv[vi] += pi;
            }
        }
        let mut flg = true;
        for &vi in vv.iter().skip(1) {
            if vi < 2 {
                flg = false;
                break;
            }
        }
        if flg {
            ans = ans.min(tmp);
        }
    }
    println!("{}", ans);
}