use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        a: [usize; n],
    }
    for &ai in a.iter() {
        if ai < x {
            x = ai;
            println!("1");
        } else {
            println!("0");
        }
    }
}