use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }
    let mut ans = 0;
    for i1 in 0..h {
        for j1 in 0..w {
            if s[i1][j1] == '#' { continue; }
            for i2 in 0..h {
                for j2 in 0..w {
                    if s[i2][j2] == '#' { continue; }
                    let mut tmp = 0;
                    for i in 0..h {
                        for j in 0..w {
                            if s[i][j] == '.' && (i1.abs_diff(i)+j1.abs_diff(j) <= d || i2.abs_diff(i)+j2.abs_diff(j) <= d) {
                                tmp += 1;
                            }
                        }
                    }
                    ans = ans.max(tmp);
                }
            }
        }
    }
    println!("{}", ans);
}