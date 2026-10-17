use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        ab: [(usize, usize); n],
    }
    let mut ans = 0;
    for i in 0..(1<<n) {
        let mut tmp_ans = 0;
        let mut tmp_w = 0;
        for j in 0..n {
            if (i>>j)&1 == 1 {
                let (a, b) = ab[j];
                tmp_w += b;
                tmp_ans += a;
            }
        }
        if tmp_w <= w {
            ans = ans.max(tmp_ans);
        }

    }
    println!("{}", ans);
}