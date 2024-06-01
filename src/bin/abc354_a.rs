use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let i = (0..31).find(|i| h < (1<<i)-1).unwrap();
    println!("{}", i);
}