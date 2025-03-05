use proconio::input;

fn main() {
    input! {
        x: isize,
    }
    if x > 0 {
        println!("{}", (x-1)/10+1);
    } else {
        println!("{}", x/10);
    }
}