use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut idx: Vec<isize> = vec![-1; t.len()];
    let mut j = 0;
    for (i, &si) in s.iter().enumerate() {
        if si == t[j] {
            idx[j] = i as isize;
            j += 1;
            if j == t.len() { break; }
        }
    }
    if idx[t.len()-1] == -1 {
        println!("{}", s.len()*(s.len()+1)/2);
        return;
    }
    let mut ans = 0;
    let mut r = idx[t.len()-1] as usize;
    for i in 0..s.len() {
        // println!("befor i: {}, r: {}, idx: {:?}, ans: {}", i, r, idx, ans);
        for j in 0..t.len() {
            if i as isize > idx[j] {
                // 先に進んでidxを更新
                let mut start = idx[j] as usize + 1;
                for jj in j..t.len() {
                    // println!("jj: {}, r: {}", jj, r);
                    let mut update = false;
                    for k in start..s.len() {
                        if t[jj] == s[k] {
                            idx[jj] = k as isize;
                            r = r.max(k);
                            start = k+1;
                            update = true;
                            break;
                        }
                    }
                    if !update {
                        r = s.len();
                        // println!("update jj: {}, r: {}", jj, r);
                    }
                }
            }
        }
        ans += r-i;
    }
    println!("{}", ans);
}