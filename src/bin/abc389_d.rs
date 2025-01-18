use proconio::input;

fn main() {
    input! {
        r: usize,
    }
    let mut ans = 0;
    let mut j = 0;
    for i in (0..r).rev() {
        let x = i as f64 + 0.5;
        let mut y = j as f64 + 0.5;
        while x*x + y*y <= (r*r) as f64 {
            ans += i*2 + 1;
            if j > 0 {
                ans += i*2 + 1;
            }
            j += 1;
            y = j as f64 + 0.5;
        }
    }
    println!("{}", ans);
}