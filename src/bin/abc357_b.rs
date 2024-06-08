use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut up_cnt = 0;
    for si in s.iter() {
        if si.is_uppercase() {
            up_cnt += 1;
        }
    }
    if s.len() - up_cnt < up_cnt {
        println!("{}", s.iter().collect::<String>().to_uppercase());
    } else {
        println!("{}", s.iter().collect::<String>().to_lowercase());
    }
}