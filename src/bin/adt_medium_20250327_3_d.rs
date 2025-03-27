use proconio::input;

fn main() {
    input! {
        a: isize,
        m: isize,
        l: isize,
        r: isize,
    }
    let diff = ((l - a) % m + m) % m;
    let ans = if diff == 0 || m - diff <= r - l {
        (r - l - (m-diff)%m) / m + 1
    } else {
        0
    };
    println!("{}", ans);
}