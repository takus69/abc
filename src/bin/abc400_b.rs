use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut x = 0;
    for i in 0..=m {
        x += n.pow(i as u32);
        if x > 1_000_000_000 {
            println!("inf");
            return;
        }
    }
    println!("{}", x);
}