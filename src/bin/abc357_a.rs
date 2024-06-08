use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: i64,
        h: [i64; n],
    }
    for i in 0..n {
        m -= h[i];
        if m < 0 {
            println!("{}", i);
            std::process::exit(0);
        }
    }
    println!("{}", n);
}