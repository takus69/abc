use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut m: usize,
        mut cv: [(usize, usize); n],
    }
    cv.sort_by(|a, b| b.1.cmp(&a.1));
    let mut ans = 0;
    let mut used: Vec<bool> = vec![false; n+1];
    for &(c, v) in &cv {
        if !used[c] {
            k -= 1;
            if m > 0 {
                m -= 1;
            }
            ans += v;
            used[c] = true;
        } else if k > m {
            k -= 1;
            ans += v;
        }
        if k == 0 { break; }
    }
    println!("{}", ans);
}