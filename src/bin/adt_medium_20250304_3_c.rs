use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut k: usize,
    }
    for i in 0..k {
        let k10 = 10_usize.pow(i as u32);
        x -= x % k10;
        let c = x % (k10*10);
        x -= c;
        if c >= 5*k10 { x += k10*10; }
    }
    println!("{}", x);
}