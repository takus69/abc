use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            x1: usize,
            y1: usize,
            r1: usize,
            x2: usize,
            y2: usize,
            r2: usize,
        }
        let diff = x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2);
        if diff <= (r1 + r2).pow(2) && r1.abs_diff(r2).pow(2) <= diff {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}