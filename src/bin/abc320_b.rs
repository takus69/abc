use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    let s2 = s.chars().rev().collect::<String>();
    for i in (1..=s.len()).rev() {
        for j in 0..=(s.len()-i) {
            if s[j..(j+i)] == s2[(s.len()-(j+i))..(s.len()-j)] {
                println!("{}", i);
                std::process::exit(0);
            }
        }
    }
}