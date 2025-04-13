use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut status = "logout";
    let mut ans = 0;
    for si in s.iter() {
        if si == "login" || si == "logout" {
            status = si;
        }
        if status == "logout" && si == "private" {
            ans += 1;
        }
    }
    println!("{}", ans);
}