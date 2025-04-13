use proconio::input;

fn main() {
    input! {
        s: usize,
    }
    let ans = if s >= 200 && s <= 299 { "Success" } else { "Failure"}; 
    println!("{}", ans);
}