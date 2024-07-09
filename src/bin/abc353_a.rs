use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    for i in 1..n {
        if h[0] < h[i] {
            println!("{}", i+1);
            std::process::exit(0);
        }
    }
    println!("-1");
}