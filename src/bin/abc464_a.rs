use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut e_cnt = 0;
    let mut w_cnt = 0;
    for &si in &s {
        if si == 'E' {
            e_cnt += 1;
        } else {
            w_cnt += 1;
        }
    }
    if e_cnt > w_cnt {
        println!("East");
    } else {
        println!("West");
    }
}