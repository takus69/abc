use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }
    for &xi in &x {
        if xi >= 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}