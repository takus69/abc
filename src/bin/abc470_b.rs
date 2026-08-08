use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut cnt: Vec<usize> = vec![0; n+1];
    for &ci in &c {
        cnt[ci] += 1;
    }
    println!("{}", n-*(cnt.iter().max().unwrap()));
}