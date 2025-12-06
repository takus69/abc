use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }
    println!("{}", (w*1000)/b+1);
}