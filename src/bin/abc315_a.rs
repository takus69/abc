use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = Vec::new();
    for si in s.iter() {
        if !['a', 'e', 'i', 'u', 'o'].contains(si) {
            ans.push(si);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}