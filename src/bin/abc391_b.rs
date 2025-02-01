use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    for a in 0..=(n-m) {
        for b in 0..=(n-m) {
            let mut flg = true;
            for i in 0..m {
                for j in 0..m {
                    if s[a+i][b+j] != t[i][j] {
                        flg = false;
                        break;
                    }
                }
            }
            if flg {
                println!("{} {}", a+1, b+1);
                std::process::exit(0);
            }
        }
    }
}