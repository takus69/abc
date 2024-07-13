use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }
    let mut max_w_t = 0;
    let mut max_l_t = 0;
    let mut w_t = 0;
    let mut l_t = 0;
    for si in s.iter() {
        if si == &'0' {
            w_t = 0;
            l_t = 0;
        } else if si == &'1' {
            if w_t < m {
                w_t += 1;
            } else {
                l_t += 1;
            }
        } else if si == &'2' {
            l_t += 1;
        }
        max_l_t = max_l_t.max(l_t);
    }
    println!("{}", max_l_t);
}