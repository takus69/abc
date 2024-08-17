use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut ha: usize,
        mut wa: usize,
        mut a: [Chars; ha],
        mut hb: usize,
        mut wb: usize,
        mut b: [Chars; hb],
        hx: usize,
        wx: usize,
        x: [Chars; hx],
    }
    // トリミング
    while a[0] == vec!['.'; wa] {
        a.remove(0);
        ha -= 1;
    }
    while b[0] == vec!['.'; wb] {
        b.remove(0);
        hb -= 1;
    }
    while a[ha-1] == vec!['.'; wa] {
        a.pop();
        ha -= 1;
    }
    while b[hb-1] == vec!['.'; wb] {
        b.pop();
        hb -= 1;
    }
    let mut flg = true;
    while flg {
        for i in 0..ha {
            if a[i][0] != '.' {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..ha {
                a[i].remove(0);
            }
            wa -= 1;
        }
    }
    let mut flg = true;
    while flg {
        for i in 0..hb {
            if b[i][0] != '.' {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..hb {
                b[i].remove(0);
            }
            wb -= 1;
        }
    }
    let mut flg = true;
    while flg {
        for i in 0..ha {
            if a[i][wa-1] != '.' {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..ha {
                a[i].pop();
            }
            wa -= 1;
        }
    }
    let mut flg = true;
    while flg {
        for i in 0..hb {
            if b[i][wb-1] != '.' {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..hb {
                b[i].pop();
            }
            wb -= 1;
        }
    }

    // はみ出す場合は黒いマスをすべて含めることができない
    if hx < ha || wx < wa || hx < hb || wx < wb {
        println!("No");
        std::process::exit(0);
    }

    // 全部の配置を試す
    for ai in 0..=(hx-ha) {
        for aj in 0..=(wx-wa) {
            for bi in 0..=(hx-hb) {
                for bj in 0..=(wx-wb) {
                    let mut c = vec![vec!['.'; wx]; hx];
                    for ai2 in 0..ha {
                        for aj2 in 0..wa {
                            c[ai+ai2][aj+aj2] = a[ai2][aj2];
                        }
                    }
                    for bi2 in 0..hb {
                        for bj2 in 0..wb {
                            if c[bi+bi2][bj+bj2] == '#' { continue; }
                            c[bi+bi2][bj+bj2] = b[bi2][bj2];
                        }
                    }
                    let mut ans: Vec<Vec<char>> = Vec::new();
                    for xi in 0..hx {
                        ans.push(c[xi][0..wx].to_vec());
                    }
                    if x == ans {
                        println!("Yes");
                        std::process::exit(0);
                    }
                }
            }
        }
    }
    println!("No");
}