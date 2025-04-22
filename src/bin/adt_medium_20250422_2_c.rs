use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }
    let mut g2 = 0;
    let mut m2 = 0;
    for _ in 0..k {
        if g2 == g {
            g2 = 0;
        } else if m2 == 0 {
            m2 = m;
        } else {
            if g - g2 < m2 {
                m2 -= g - g2;
                g2 = g;
            } else {
                g2 += m2;
                m2 = 0;
            }
        }
    }
    println!("{} {}", g2, m2);

}