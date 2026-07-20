use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    // xが書かれてあるカードの集合 Si
    // (x, y)のうち片方だけ含むカードが存在「しない」 <=> すべてのカードが、(x, y)両方かかれている or (x, y)どちらもかかれていない
    // Sx = Sy と同値、つまり、すべてのSiが相異なることが条件
    // Siの選び方は2^n <= k なら達成可能
    // 1からnのカードからi枚選ぶ方法は、nCi 通り
    // コストの大きい方（大きい数字）から順に nCi の小さい方から割り当てていけばよい

    for _ in 0..t {
        input! {
            n: usize,
            mut k: usize,
        }
        // 達成可能かチェック
        let mut k2 = k-1;
        let mut flg = true;
        for _ in 0..n {
            k2 /= 2;
            if k2 == 0 {
                flg = false;
                break;
            }
        }
        if flg {
            println!("-1");
            continue;
        }
        // 桁数取得
        k -= 1;
        let mut d = 1;
        while k >= 10usize.pow(d) {
            d += 1;
        }
        let mut ans = 0;
        let mut comb = 1;
        let mut den = n;
        for i in 1..=n {
            comb *= den;
            comb /= i;
            let mut comb2 = comb;
            while comb2 > 0 && k > 0 {
                let cnt = k - 10usize.pow(d-1) + 1;
                if comb2 > cnt {
                    ans += i * d as usize * cnt;
                    k -= cnt;
                    comb2 -= cnt;
                    d -= 1;
                    continue;
                } else {
                    ans += i * d as usize * comb2;
                    k -= comb2;
                    comb2 = 0;
                }
            }
            den -= 1;
            if k == 0 { break; }
        }
        println!("{}", ans);
    }
}