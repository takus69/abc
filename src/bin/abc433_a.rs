use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    }
    if x >= z*y && (x-z*y)%(z-1)==0 {
        println!("Yes");
    } else {
        println!("No");
    }
}