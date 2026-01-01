use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for &ai in a.iter() {
        ans ^= ai;
    }
    if ans == 0 {
        println!("Second");

    } else {
        println!("First");
    }
}