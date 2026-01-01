use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;
    for ai in 1..=100 {
        for bi in 1..=100 {
            let mut cnt = 0;
            for i in 0..n {
                let (ai2, bi2) = ab[i];
                if ai <= ai2 && ai2 <= ai+k && bi <= bi2 && bi2 <= bi+k {
                    cnt += 1;
                }
            }
            ans = ans.max(cnt);
        }
    }
    println!("{}", ans);
}