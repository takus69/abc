use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans =0;
    for l in 0..n {
        for r in l..n {
            let mut sum = 0;
            for i in l..=r {
                sum += a[i];
            }
            let mut flg = true;
            for i in l..=r {
                if sum%a[i] ==0 {
                    flg = false;
                    break;
                }
            }
            if flg { ans += 1; }
        }
    }
    println!("{}", ans);
}