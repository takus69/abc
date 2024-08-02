use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort();
    let mut ans = 1;
    let mut sum_b: usize = ab.iter().map(|x| x.1).sum();
    let mut i = 0;
    loop {
        if sum_b <= k {
            break;
        }
        let (a, b) = ab[i];
        ans = a+1;
        sum_b -= b;
        i += 1;
    }
    println!("{}", ans);
}