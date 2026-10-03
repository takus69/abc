use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    for i in 0..n {
        println!("{}", m/n+ if m%n > i { 1 } else { 0 });
    }
}