use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let ans = match x {
        0..40 => { 40 - x },
        40..70 => { 70 - x },
        70..90 => { 90 - x },
        _ => { println!("expert");return; }
    };
    println!("{}", ans);
}