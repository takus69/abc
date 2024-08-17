use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    }
    for i in (0..x.len()).rev() {
        if x[i] == '0' {
            x.pop();
        } else {
            break;
        }
    }
    if x[x.len()-1] == '.' {
        x.pop();
    }
    println!("{}", x.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}