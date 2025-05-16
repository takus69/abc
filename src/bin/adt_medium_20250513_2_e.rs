use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }
    let mut l = 0;
    let mut now_m = m;
    let mut now_l = 0;
    for &si in s.iter() {
        match si {
            '0' => {
                now_m = m;
                now_l = l;
            },
            '1' => {
                if now_m > 0 {
                    now_m -= 1;
                } else if now_l > 0 {
                    now_l -= 1;
                } else {
                    l += 1;
                }
            },
            '2' => {
                if now_l > 0 {
                    now_l -= 1;
                } else {
                    l += 1;
                }
            },
            _ => {},
        } 
    }
    println!("{}", l);
}