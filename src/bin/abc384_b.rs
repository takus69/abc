use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: isize,
        da: [(usize, isize); n],
    }
    for &(di, ai) in da.iter() {
        if di == 1 && r >= 1600 && r <= 2799 {
            r += ai;
        } else if di == 2 && r >= 1200 && r <= 2399 {
            r += ai;
        }
    }
    println!("{}", r);
}