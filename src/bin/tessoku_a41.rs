use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut pre =' ';
    let mut cnt = 0;
    for &si in s.iter() {
        if pre != si {
            cnt = 1;
            pre = si;
        } else {
            cnt += 1;
            if cnt == 3 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}