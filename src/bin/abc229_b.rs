use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }
    while a > 0 && b > 0 {
        if a%10+b%10 >= 10 {
            println!("Hard");
            return;
        }
        a /= 10;
        b /= 10;
    }
    println!("Easy");
}