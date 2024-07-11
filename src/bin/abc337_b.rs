use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let c = ['A', 'B', 'C'];
    let mut i = 0;
    let mut ans = "Yes";
    for si in s.iter() {
        if si == &c[i] {
            continue;
        } else {
            i += 1;
            if i < 3 && si == &c[i] {
                continue;
            } else {
                i += 1;
                if i < 3 && si == &c[i] {
                    continue;
                } else {
                    ans = "No";
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}