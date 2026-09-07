use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut imos: Vec<isize> = vec![0; t+1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    let mut prefix: Vec<isize> = vec![0; t+1];
    prefix[0] = imos[0];
    for i in 0..t {
        prefix[i+1] = prefix[i] + imos[i+1];
    }
    for i in 0..t {
        println!("{}", prefix[i]);
    }
}