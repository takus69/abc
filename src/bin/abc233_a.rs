use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let ans = if x >= y {
        0
    } else {
        (y-x-1)/10+1
    };
    println!("{}", ans);
}