use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let mut ans = (x+y)%12;
    if ans==0 { ans = 12; }
    println!("{}", ans);
}