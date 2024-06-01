use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut c = Vec::new();
    let mut a = Vec::new();
    let mut r = Vec::new();
    for _ in 0..m {
        input! {
            ci: usize,
            ai: [usize; ci],
            ri: char,
        }
        c.push(ci);
        a.push(ai);
        r.push(ri);
    }
    let mut ans = 0;
    for mask in 0..(1<<n) {  // bit全探索
        let mut ok = true;
        for i in 0..m {  // 試行を順に試す
            let mut cnt = 0;
            for ai in a[i].iter() {  // 試行の鍵を順に試す
                cnt += mask >> (ai-1) & 1;  // 右シフトして、対象の場所に1が立っているかチェック(立っていると+1, 立ってないと+0)
            }
            ok &= (cnt >= k) == (r[i] == 'o');
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}