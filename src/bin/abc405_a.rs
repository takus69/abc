use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
    }
    if (x == 1 && 1600 <= r && r <= 2999) || (x==2 && 1200 <= r && r <= 2399) {
        println!("Yes");
    } else {
        println!("No");
    }
}