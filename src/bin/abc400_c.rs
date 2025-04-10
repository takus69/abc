use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut max_a: usize = 0;
    while n >> max_a > 1 {
        max_a += 1;
    }

    let mut ans: usize = 0;
    for a in 1..=max_a {
        let b2: usize = n / 2_usize.pow(a as u32);
        let b: usize = (b2 as f64).sqrt() as usize;
        ans += b/2+b%2;
    }

    println!("{}", ans);
}