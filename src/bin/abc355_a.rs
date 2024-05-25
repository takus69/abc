use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let mut ans = -1;
    if a != b {
        for d in 1..=3 {
            if a != d && b != d {
                ans = d;
            }
        }
    }
    println!("{}", ans);
}