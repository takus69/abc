use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let mut n1 = 0;
    let mut n2 = 0;
    let mut n3 = 0;
    for i in 0..n.len() {
        match n[i] {
            '1' => { n1 += 1;},
            '2' => { n2 += 1;},
            '3' => { n3 += 1;},
            _ => {},
        }
    }
    if n1 == 1 && n2 == 2 && n3 == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}