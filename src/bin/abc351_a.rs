use proconio::input;

fn main() {
    input! {
        a: [i32; 9],
        b: [i32; 8],
    }
    let mut ans = 0;
    for ai in a {
        ans += ai;
    }
    for bi in b {
        ans -= bi;
    }
    ans += 1;
    println!("{}", ans);
}