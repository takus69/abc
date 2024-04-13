use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n-1],
    }
    let sum: i32 = a.iter().sum();
    println!("{}", -sum);
}