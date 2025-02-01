use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(usize, usize); n],
        q: usize,
        ta: [(usize, usize); q],
    }

    let mut xy2 = Vec::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        xy2.push((i, x-1, y-1));
    }
    xy2.sort_by(|a, b| a.2.cmp(&b.2));  // yで昇順にソート
    let mut del_time = Vec::new();  // (時間に、何列削除したか)
    let mut row_cnt: Vec<usize> = Vec::new();  // <下から何行目、何個処理したか>
    let mut col_cnt: Vec<usize> = vec![0; w];  // 何列目を何個処理したか、下から何個目
    let mut a_row: Vec<usize> = vec![0; n];

    for &(i, xi, yi) in xy2.iter() {
        let c: usize = col_cnt[xi] + 1;
        col_cnt[xi] = c;
        a_row[i] = c;
        if row_cnt.len() < c {
            row_cnt.push(0);
        }
        row_cnt[c-1] += 1;
        if row_cnt[c-1] == w {
            let (pre_t, _) = if del_time.len() > 0 { del_time.last().unwrap() } else { &(0, 0) };
            // println!("c: {}, w: {}, pre_t+1: {}, yi+1: {}", c, w, pre_t+1, yi+1);
            del_time.push(((pre_t+1).max(yi+1), c));
        }
    }
    // println!("{:?}", xy2);
    // println!("col_cnt: {:?}, row_cnt: {:?}, del_time: {:?}", col_cnt, row_cnt, del_time);

    for &(t, a) in ta.iter() {
        let r = a_row[a-1];
        let (d_time, _) = if del_time.len() >= r { del_time[r-1] } else { (usize::MAX, 0) };
        // println!("t: {}, a: {}, r: {}, d_time: {}", t, a, r, d_time);
        if t >= d_time {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}