use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, usize); m],
    }
    let mut ans: usize = (1<<30)-1;
    for k in (0..30).rev() {
        // 上位の桁からk桁目が0で到達可能かを判定
        let mut dsu: Dsu = Dsu::new(n+1);
        ans &= !(1 << k);  // k桁目を0にする
        for &(a, b, w) in &uvw {
            if (ans | w) == ans {  // 上位の決まった桁は一致して、k桁目が0のパスだけを抜き出す
                dsu.merge(a, b);
            }
        }
        if !dsu.same(1, n) {  // 到達可能でなければk桁目は1確定
            ans |= 1 << k;
        }
    }
    println!("{}", ans);
}