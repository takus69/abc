use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    if l == 1 && r == 0 {
        println!("Yes");
    } else if l == 0 && r == 1 {
        println!("No");
    } else {
        println!("Invalid");
    }
}