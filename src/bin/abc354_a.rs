use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let mut h2 = 1;
    let mut i2 = 1;
    let mut ans = 1;
    while h >= h2 {
        i2 *= 2;
        h2 += i2;
        ans += 1;
        // println!("{}日目, h2: {}cm", ans, h2);
    }
    println!("{}", ans);
}