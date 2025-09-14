use proconio::input;

fn main() {
    input! {
        mut x: usize,
        c: usize,
    }
    let mut ans = 0;
    while x >= 0 {
        if x >= 1000+c {
            x -= 1000+c;
            ans += 1000;
        } else {
            break;
        }
    }
    println!("{}", ans);
}