use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", a.iter().skip(n/2).sum::<usize>());
}