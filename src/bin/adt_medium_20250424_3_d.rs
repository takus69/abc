use proconio::input;

fn main() {
    input! {
        mut x: usize,
        k: usize,
    }
    for i in 0..k {
        let t = 10_usize.pow((i+1) as u32);
        let x2 = x % t;
        x -= x2;
        if x2 >= 5 * (t/10) {
            x += t;
        }
    }
    println!("{}", x);
}