use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.sort();
    for _ in 0..q {
        input! {
            x: usize,
        }
        let cnt = a.partition_point(|&ai| ai < x);
        println!("{}", n-cnt);
    }
}