use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    }
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        if a == 1 {
            println!("{}", b*w);
            h -= b;
        } else {
            println!("{}", b*h);
            w -= b;
        }
    }
}