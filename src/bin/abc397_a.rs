use proconio::input;

fn main() {
    input! {
        x: f64,
    }
    let ans = if x >= 38.0 { 1 } else if x >= 37.5 { 2 } else { 3 };
    println!("{}", ans);
}