use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans: Vec<usize> = vec![0, 0, 0];
    for &ai in &a {
        let mut ai = ai - (ai/1000)*1000;
        for i in 0..3 {
            let base = 10usize.pow((i+1) as u32);
            if (ai%base)/(base/10)==0 { continue; }
            ans[i] += 10-(ai%base)/(base/10);
            ai += (10-(ai%base)/(base/10))*(base/10);
        }
    }
    println!("{}", ans.iter().join(" "));
}