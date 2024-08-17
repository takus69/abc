use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        mut c: usize,
    }
    if c < b { c += 24; }
    let mut ans = "Yes";
    if b < a && a < c {
        ans = "No";
    } else if b < a+24 && a+24 < c {
        ans = "No";
    }
    println!("{}", ans);
}