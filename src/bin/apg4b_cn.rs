use proconio::input;

fn main() {
    input! {
        mut x: i32,
        a: i32,
        b: i32,
    }
    x += 1;
    println!("{}", x);
    x *= a+b;
    println!("{}", x);
    x *= x;
    println!("{}", x);
    x -= 1;
    println!("{}", x);
}