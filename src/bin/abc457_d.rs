use proconio::input;

fn main() {
    input! {
        n: u128,
        k: u128,
        a: [u128; n],
    }
    let mut ng: u128 = 2*1_000_000_000_000_000_000_000_000_000;
    let mut ok: u128 = 0;
    while ok+1 < ng {
        let m = (ok+ng)/2;
        let mut k2 = 0;
        for (i, &ai) in a.iter().enumerate() {
            if ai >= m { continue; }
            k2 += (m-ai-1)/(i as u128 +1)+1;
        }
        if k < k2 {
            ng = m;
        } else {
            ok = m;
        }
    }
    println!("{}", ok);
}