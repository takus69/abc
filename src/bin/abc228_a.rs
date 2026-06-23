use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    }
    if (s < t && s <= x && x < t) || (s > t && (s <= x || x < t)) {
        println!("Yes");
    } else {
        println!("No");
    }
}