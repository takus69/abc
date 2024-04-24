use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    print!("A:");
    for _ in 0..a { print!("]"); }
    println!("");
    print!("B:");
    for _ in 0..b { print!("]"); }
    println!("");
}