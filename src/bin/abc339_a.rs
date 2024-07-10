use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s: Vec<&str> = s.split('.').collect();
    println!("{}", s[s.len()-1]);
}