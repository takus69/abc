use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut l: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            li: usize,
        }
        input! {
            ll: [usize; li],
        }
        l.push(ll);
    }
    input! {
        x: usize,
        y: usize,
    }
    println!("{}", l[x-1][y-1]);
}