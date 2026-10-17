use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    let mut ans: Vec<usize> = vec![0; n];
    for tj in &t {
        let mut flg2 = false;
        let mut cnt = 0;
        let mut tmp_i = 0;
        for i in 0..n {
            // println!("tj: {:?}, i: {}, si: {:?}", tj, i, s[i]);
            if tj == &s[i] {
                ans[i] += 1;
                flg2 = true;
                break;
            }

            let mut flg = false;
            for k in 0..s[i].len() {
                let mut tmp_si = s[i].clone();
                for c in "abcdefghijklmnopqrstuvwxyz".chars() {
                    tmp_si[k] = c;
                    if &tmp_si == tj {
                        cnt += 1;
                        tmp_i = i;
                        flg = true;
                        break;
                    }   
                }
                if flg { break; }
            }
        }
        // println!("cnt: {}, tmp_i: {}", cnt, tmp_i);
        if !flg2 && cnt == 1 {
            ans[tmp_i] += 1;
        }
    }
    for a in &ans {
        println!("{}", a);
    }
}