use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut cnt: Vec<usize> = vec![0; n+1];
    for &(a, b) in &ab {
        cnt[b] += 1;
        cnt[a] += 1;
    }
    for i in 1..=n {
        if cnt[i] == n-1 {
            println!("Yes");
            return
        }
    }
    println!("No");
}