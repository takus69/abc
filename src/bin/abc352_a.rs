use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
    }
    let l = x.min(y);
    let r = x.max(y);
    if l < z && z < r {
        println!("Yes");
    } else {
        println!("No");
    }
}