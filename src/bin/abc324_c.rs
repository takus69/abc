use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    for (i, si) in s.iter().enumerate() {
        let diff = si.len() as i64 - t.len() as i64;
        if diff.abs() >  1 { continue; }
        let mut cnt = 0;
        for j in 0..t.len().min(si.len()) {
            if cnt > 1 { break; }
            let mut jt = j;
            let mut js = j;
            if cnt > 0 {
                match diff {
                    1 => { js += 1; },
                    -1 => { jt += 1; },
                    _ => {},
                }
            }
            if si[js] != t[jt] {
                cnt += 1;
                if cnt > 1 { break; }
                if diff > 0 {
                    js += 1;
                    if si[js] != t[jt] { cnt += 1; }
                } else if diff < 0 {
                    jt += 1;
                    if si[js] != t[jt] { cnt += 1; }
                }
            }
        }
        if cnt <= 1 {
            ans.push(i+1);
        }
    }
    let k = ans.len();
    println!("{}", k);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}