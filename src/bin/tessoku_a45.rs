use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: char,
        a: Chars,
    }
    let (mut w_cnt, mut r_cnt, mut b_cnt) = (0usize, 0usize, 0usize);
    for &ai in a.iter() {
        match ai {
            'W' => { w_cnt += 1; },
            'R' => { r_cnt += 1; },
            'B' => { b_cnt += 1; },
            _ => {},
        }
    }
    let diff: usize = r_cnt.abs_diff(b_cnt);
    let mut r_flg = if r_cnt > b_cnt { true } else { false };
    let mut ans = ' ';
    if diff%3 == 0 {
        ans = 'W'
    } else {
        if diff%3 == 2 {
            r_flg = !r_flg;
        }
        ans = if r_flg { 'R' } else { 'B' };
    }

    if ans == c {
        println!("Yes");
    } else {
        println!("No");
    }

}