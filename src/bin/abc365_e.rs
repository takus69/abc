use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    // ビットごとにXORの1の個数の累積和を求める
    let mut sum_cnt: Vec<Vec<usize>> = Vec::new();
    sum_cnt.push(vec![0; 30]);
    let mut xor = 0;
    for i in 0..n {
        xor ^= a[i];
        // println!("i: {}, xor: {}", i, xor);
        sum_cnt.push(vec![0; 30]);
        for j in 0..30 {
            let pre_cnt = sum_cnt[sum_cnt.len()-2][j];
            // println!("j: {}, pre_cnt: {}, xor>>j: {}", j, pre_cnt, xor>>j);
            if (xor>>j & 1)==1 {
                sum_cnt[i+1][j] = pre_cnt+1;
            } else {
                sum_cnt[i+1][j] = pre_cnt;
            }
        }
    }
    // println!("{:?}", sum_cnt);

    let mut ans = 0;
    for j in 0..30 {
        let cnt1 = sum_cnt[n][j];
        let cnt0 = n - cnt1;
        // println!("cnt1: {}, cnt0: {}", cnt1, cnt0);
        ans += cnt1 * (cnt0+1) * (1<<j);
    }
    ans -= a.iter().sum::<usize>();
    println!("{}", ans);
}