use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut rows: HashMap<(char, usize), usize> = HashMap::new();
    let mut cols: HashMap<(char, usize), usize> = HashMap::new();
    let abc = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    for i in 0..h {
        let mut tmp: HashMap<char, usize> = HashMap::new();
        for ci in c[i].iter() {
            let e = tmp.entry(*ci).or_insert(0);
            *e += 1;
        }
        for (ci, cnt) in tmp.iter() {
            let e = rows.entry((*ci, *cnt)).or_insert(0);
            *e += 1;
        }
    }
    for j in 0..w {
        let mut tmp: HashMap<char, usize> = HashMap::new();
        for i in 0..h {
            let ci = c[i][j];
            let e = tmp.entry(ci).or_insert(0);
            *e += 1;
        }
        for (ci, cnt) in tmp.iter() {
            let e = cols.entry((*ci, *cnt)).or_insert(0);
            *e += 1;
        }
    }
    // println!("rows: {:?}, cols: {:?}", rows, cols);

    let mut row_cnt = h;
    let mut col_cnt = w;
    let mut del_row_char: HashMap<char, usize> = HashMap::new();
    let mut del_col_char: HashMap<char, usize> = HashMap::new();
    let mut flg = true;
    while flg {
        flg = false;
        let mut tmp_del_row_cnt = 0;
        let mut tmp_del_col_cnt = 0;
        let mut tmp_del_row_char: HashMap<char, usize> = HashMap::new();
        let mut tmp_del_col_char: HashMap<char, usize> = HashMap::new();
        for ci in abc.iter() {
            if col_cnt < 2 { break; }
            let del_cnt = if del_col_char.contains_key(ci) {
                del_col_char[ci]
            } else { 0 };
            if rows.contains_key(&(*ci, col_cnt+del_cnt)) {
                let cnt = rows.get(&(*ci, col_cnt+del_cnt)).unwrap();
                // println!("rows: ci: {}, col_cnt: {}, del_cnt: {}, cnt: {}", ci, col_cnt, del_cnt, cnt);
                tmp_del_row_cnt += cnt;
                let e = tmp_del_row_char.entry(*ci).or_insert(0);
                *e += cnt;
                flg = true;
                rows.remove(&(*ci, col_cnt+del_cnt));
            }
        }
        for ci in abc.iter() {
            if row_cnt < 2 { break; }
            let del_cnt = if del_row_char.contains_key(ci) {
                del_row_char[ci]
            } else { 0 };
            if cols.contains_key(&(*ci, row_cnt+del_cnt)) {
                let cnt = cols.get(&(*ci, row_cnt+del_cnt)).unwrap();
                // println!("cols: ci: {}, row_cnt: {}, del_cnt: {}, cnt: {}", ci, row_cnt, del_cnt, cnt);
                tmp_del_col_cnt += cnt;
                let e = tmp_del_col_char.entry(*ci).or_insert(0);
                *e += cnt;
                flg = true;
                cols.remove(&(*ci, row_cnt+del_cnt));
            }
        }
        row_cnt -= tmp_del_row_cnt;
        col_cnt -= tmp_del_col_cnt;
        for (ci, cnt) in tmp_del_row_char.iter() {
            let e = del_row_char.entry(*ci).or_insert(0);
            *e += cnt;
        }
        for (ci, cnt) in tmp_del_col_char.iter() {
            let e = del_col_char.entry(*ci).or_insert(0);
            *e += cnt;
        }
        // println!("flg: {:?}, row_cnt: {}, col_cnt: {}", flg, row_cnt, col_cnt);
    }
    println!("{}", row_cnt*col_cnt);
}