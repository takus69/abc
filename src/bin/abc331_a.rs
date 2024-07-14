use proconio::input;

fn main() {
    input! {
        M: usize,
        D: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize,
    }
    d += 1;
    if d == D+1 {
        d = 1;
        m += 1;
    }
    if m == M+1 {
        m = 1;;
        y += 1;
    }
    println!("{} {} {}", y, m, d);
}