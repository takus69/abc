use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }
    a.sort();
    for _ in 0..q {
        input! {
            x: usize,
        }
        println!("{}", a.partition_point(|&y| y < x))
    }
}