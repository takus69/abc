use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }
    let mut gg = 0;
    let mut mm = 0;
    for _ in 0..k {
        if gg == g {
            gg = 0;
        } else if mm == 0 {
            mm = m;
        } else {
            if g - gg < mm {
                mm -= g - gg;
                gg = g;
            } else {
                gg += mm;
                mm = 0;
            }
        }
    }
    println!("{} {}", gg, mm);
}