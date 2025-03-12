use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    fn add_i(base2: &Vec<Vec<char>>, ii: usize, r: &Vec<char>, c: &Vec<char>) -> (bool, Vec<Vec<char>>) {
        let s = ['A', 'B', 'C'];
        let n = base2.len();
        for perm in (0..n).into_iter().permutations(3) {
            let mut base = base2.clone();
            for (si, &j) in perm.iter().enumerate() {
                base[ii][j] = s[si];
            }

            // 整合チェック
            let mut flg = true;
            for (i, &ri) in r.iter().enumerate() {
                for j in 0..n {
                    if base[i][j] == ri {
                        break;
                    } else if base[i][j] == '.' {
                        continue;
                    } else {
                        flg = false;
                        break;
                    }
                }
            }
            for (j, &ci) in c.iter().enumerate() {
                for i in 0..n {
                    if base[i][j] == ci {
                        break;
                    } else if base[i][j] == '.' {
                        continue;
                    } else {
                        flg = false;
                        break;
                    }
                }
            }
            for j in 0..n {
                let (mut a_cnt, mut b_cnt, mut c_cnt) = (0, 0, 0);
                for i in 0..n {
                    match base[i][j] {
                        'A' => { a_cnt += 1; },
                        'B' => { b_cnt += 1; },
                        'C' => { c_cnt += 1; },
                        _ => { },
                    }
                }
                if a_cnt > 1 || b_cnt > 1 || c_cnt > 1 {
                    flg = false;
                }
            }

            if flg {
                if ii+1 < n {
                    (flg, base) = add_i(&base, ii+1, r, c);
                }
                if flg {
                    return (flg, base);
                }
            }
        }

        (false, base2.clone())
    }

    let base = vec![vec!['.'; n]; n];
    let (flg, base) = add_i(&base, 0, &r, &c);
    if flg {
        println!("Yes");
        for b in base.iter() {
            println!("{}", b.iter().join(""));
        }
    } else {
        println!("No");
    }
}