use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a == 0 {
        println!("Silver");
    } else if b == 0 {
        println!("Gold");
    } else {
        println!("Alloy");
    }
}