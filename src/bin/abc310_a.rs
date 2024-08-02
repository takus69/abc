use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    }
    let min_d = d.iter().min().unwrap();
    println!("{}", p.min(q+min_d));
}