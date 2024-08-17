use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            let mut tmp = s[i].clone();
            tmp.extend(s[j].clone());
            if tmp == tmp.clone().into_iter().rev().collect::<Vec<char>>() {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }
    println!("No");
}