use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for k in 0..=60 {
        if 2usize.pow(k as u32) > n {
            println!("{}", k-1);
            break;
        }
    }
}