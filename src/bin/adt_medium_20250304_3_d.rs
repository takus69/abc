use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: u32,
        mut a: Chars,
        mut b: Chars,
    }
    a.reverse();
    b.reverse();
    let mut a2 = 0;
    let mut b2 = 0;
    for i in 0..a.len() {
        let ai: usize = a[i].to_string().parse().unwrap();
        a2 += ai * k.pow(i as u32) as usize;
    }
    for i in 0..b.len() {
        let bi: usize = b[i].to_string().parse().unwrap();
        b2 += bi * k.pow(i as u32) as usize;
    }

    let ans = a2 * b2;
    println!("{}", ans);
}