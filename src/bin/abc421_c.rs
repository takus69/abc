use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut a_index: Vec<usize> = Vec::new();
    let mut b_index: Vec<usize> = Vec::new();
    for (i, &si) in s.iter().enumerate() {
        if si == 'A' {
            a_index.push(i);
        } else {
            b_index.push(i);
        }
    }
    let mut ans_a = 0;
    let mut ans_b = 0;
    for i in 0..n {
        let ai = a_index[i];
        ans_a += ai.abs_diff(2*i);
        let bi = b_index[i];
        ans_b += bi.abs_diff(2*i);
    }
    println!("{}", ans_a.min(ans_b));
}