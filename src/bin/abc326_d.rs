use proconio::{input, marker::Chars};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }

    fn check(mass: &Vec<Vec<char>>, i: usize, c: &Vec<char>, r: &Vec<char>) -> bool {
        // i行目までが条件に合っているかを確認する
        let n = mass.len();
        for j in 0..n {
            let mut cnt = HashMap::new();
            for i2 in 0..i {
                let e = cnt.entry(mass[i2][j]).or_insert(0);
                *e += 1;
            }
            for a in ['A', 'B', 'C'] {
                let e = cnt.entry(a).or_insert(0);
                if *e > 1 { return false; }
            }
        }
        for j in 0..n {
            for i in 0..n {
                if mass[i][j] == '.' {
                    continue;
                } else {
                    if mass[i][j] == c[j] {
                        break;
                    } else {
                        return false;
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if mass[i][j] == '.' {
                    continue;
                } else {
                    if mass[i][j] == r[i] {
                        break;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn add_row(mass: Vec<Vec<char>>, i: usize, r: &Vec<char>, c: &Vec<char>) -> bool {
        // i+1行目を追加する
        let abc = ['A', 'B', 'C'];
        if i == mass.len() { return true; }
        let n = mass.len();
        if mass[i][0] == '.' {
            // 1列目が'.'のため3つ追加
            for perm in (1..n).combinations(3) {
                let mut mass2 = mass.clone();
                mass2[i][perm[0]] = r[i];
                let mut add: Vec<char> = Vec::new();
                for a in abc {
                    if a != r[i] { add.push(a); }
                }
                mass2[i][perm[1]] = add[0];
                mass2[i][perm[2]] = add[1];
                if check(&mass2, i+1, &c, &r) {
                    if add_row(mass2.clone(), i+1, r, c) {
                        println!("Yes");
                        for m in mass2.iter() {
                            println!("{}", m.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
                        }
                        std::process::exit(0);
                    }
                }
                mass2[i][perm[1]] = add[1];
                mass2[i][perm[2]] = add[0];
                if check(&mass2, i+1, &c, &r) {
                    if add_row(mass2.clone(), i+1, r, c) {
                        println!("Yes");
                        for m in mass2.iter() {
                            println!("{}", m.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
                        }
                        std::process::exit(0);
                    }
                }
            }
        } else {
            // 1列目が'.'でないため2つ追加
            for perm in (1..n).combinations(2) {
                let mut mass2 = mass.clone();
                let c0 = mass2[i][0];
                let mut add: Vec<char> = Vec::new();
                for a in abc {
                    if a != c0 { add.push(a); }
                }
                mass2[i][perm[0]] = add[0];
                mass2[i][perm[1]] = add[1];
                if check(&mass2, i+1, &c, &r) {
                    if add_row(mass2.clone(), i+1, r, c) {
                        println!("Yes");
                        for m in mass2.iter() {
                            println!("{}", m.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
                        }
                        std::process::exit(0);
                    }
                }
                mass2[i][perm[0]] = add[1];
                mass2[i][perm[1]] = add[0];
                if check(&mass2, i+1, &c, &r) {
                    if add_row(mass2.clone(), i+1, r, c) {
                        println!("Yes");
                        for m in mass2.iter() {
                            println!("{}", m.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
                        }
                        std::process::exit(0);
                    }
                }
            }
        }

        false
    }

    // 1行目のパターン
    for perm in (0..n).combinations(3) {
        let mut mass: Vec<Vec<char>> = vec![vec!['.'; n]; n];
        let mut row: Vec<char> = Vec::new();
        for p in perm.iter() {
            mass[0][*p] = c[*p];
            row.push(c[*p]);
        }
        row.sort();
        if row != vec!['A', 'B', 'C'] {
            continue;
        }
        // 1列目のパターン
        for perm2 in (0..n).combinations(3) {
            if (mass[0][0] == '.' && perm2[0] == 0)
                || (r[perm2[0]] != c[0])
                || (r[0] != c[perm[0]])
                || (mass[0][0] != '.' && perm2[0] != 0)
                || (mass[0][0] != '.' && perm2[0] == 0 && r[0] != c[0]) {
                continue;
            }
            let mut mass2 = mass.clone();
            for p2 in perm2.iter() {
                mass2[*p2][0] = r[*p2];
            }
            // 各行のパターン
            add_row(mass2.clone(), 1, &r, &c);
            // 各行の残りの列のパターン.上の行にすでにあるとNG
        }
    }
    println!("No");
}