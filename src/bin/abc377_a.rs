use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    if s.contains(&'A') && s.contains(&'B') && s.contains(&'C') {
        println!("Yes");
    } else {
        println!("No");
    }
}