use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, char); m],
    }
    let mut cnt: Vec<usize> = vec![0; n];
    for i in 0..m {
        let (a, b) = ab[i];
        if b == 'M' {
            cnt[a-1] += 1;
        }
        if b == 'M' && cnt[a-1] == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}