use proconio::input;

fn main() {
    input! {
        y: usize,
    }
    let mut ans = 365;
    if (y%4==0 && y%100!=0) || y%400==0 {
        ans = 366;
    }
    println!("{}", ans);
}