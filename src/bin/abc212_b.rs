use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }
    let mut ans = "Strong";
    if x[0]==x[1] && x[1]==x[2] && x[2]==x[3] {
        ans = "Weak";
    }
    let mut flg = true;
    for i in 0..3 {
        if (x[i].to_digit(10).unwrap()+1)%10 != x[i+1].to_digit(10).unwrap() {
            flg = false;
        }
    }
    if flg { ans = "Weak"; }
    println!("{}", ans);
}