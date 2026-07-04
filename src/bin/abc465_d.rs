use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut x: usize,
            mut y: usize,
            k: usize,
        }
        let mut ans = 0;
        while x != y {
            ans += 1;
            if x > y {
                x /= k;
            } else {
                y /= k;
            }
        }
        println!("{}", ans);
    }
}