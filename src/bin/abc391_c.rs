use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ans = 0;  // 複数の鳩がいる箱の数
    let mut ph = Vec::new();  // 鳩がどの箱にいるか
    let mut h_cnt = vec![1; n];  // 箱の鳩の数
    for i in 0..n {
        ph.push(i);
    }
    for _ in 0..q {
        input! {
            qi: usize,
        }
        if qi == 1 {
            input! {
                mut p: usize,
                mut h: usize,
            }
            p -= 1;
            h -= 1;
            h_cnt[ph[p]] -= 1;
            if h_cnt[ph[p]] == 1 { ans -= 1; }
            h_cnt[h] += 1;
            if h_cnt[h] == 2 { ans += 1; }
            ph[p] = h;
        } else {
            println!("{}", ans);
        }
    }
}