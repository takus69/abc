use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [usize; m],
        ab: [(usize, usize); n],
    }
    let mut ans = 0;
    for &(ai, bi) in ab.iter() {
        if c[ai-1] > bi {
            c[ai-1] -= bi;
            ans += bi;
        } else {
            ans += c[ai-1];
            c[ai-1] = 0;
        }
    }

    println!("{}", ans);
}