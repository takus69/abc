use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut flg = true;
        let s_i = i.to_string();
        for j in 1..s_i.len() {
            if s_i[0..1] != s_i[j..(j+1)] {
                flg = false;
                break;
            }
        }
        if !flg {
            continue;
        }
        // println!("m: {}", i);
        let di = d[i-1];
        for dd in 1..=di {
            flg = true;
            let dd = dd.to_string();
            for j in 0..dd.len() {
                if dd[j..(j+1)] != s_i[0..1] {
                    flg = false;
                    break;
                }
            }
            if flg {
                ans += 1;
                // println!("d: {}, flg: {}", dd, flg);
            }
        }
    }
    println!("{}", ans);
}