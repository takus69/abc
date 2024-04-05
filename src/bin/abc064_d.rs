use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: i32,
        s: Chars,
    }
    let mut l_cnt: i32 = 0;
    let mut l_cnt_max: i32 = 0;
    let mut r_cnt: i32 = 0;
    let mut r_cnt_max: i32 = 0;
    for ss in &s {
        if ss == &')' {
            l_cnt += 1;
            l_cnt_max = l_cnt_max.max(l_cnt);
        } else {
            l_cnt -= 1;
        }
    }
    for ss in s.iter().rev() {
        if ss == &'(' {
            r_cnt += 1;
            r_cnt_max = r_cnt_max.max(r_cnt);
        } else {
            r_cnt -= 1;
        }
    }
    println!("{}", '('.to_string().repeat(l_cnt_max as usize) + &s.iter().collect::<String>() + &')'.to_string().repeat(r_cnt_max as usize));

}
