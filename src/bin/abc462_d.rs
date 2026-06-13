use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        st: [(usize, usize); n],
    }
    let mut max_t = 0;
    for &(_, t) in &st {
        max_t = max_t.max(t);
    }
    let mut cnt: Vec<isize> = vec![0; max_t+1];
    // println!("max_t: {}", max_t);
    for &(s, t) in &st {
        if s+d > t { continue; }
        cnt[s] += 1;
        cnt[t-d+1] -= 1;
    }
    for i in 0..max_t {
        cnt[i+1] += cnt[i]
    }
    let mut ans: isize = 0;
    for &c in &cnt {
        if c > 1 {
            ans += c*(c-1)/2;
        }
    }
    println!("{}", ans);
}