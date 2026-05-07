use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut r: [usize; n],
    }

    // 端の木を削除
    for i in 0..n {
        if let Some(&si) = s.last() {
            if si == '#' { s.pop();r.pop(); } else { break; }
        }
    }
    s.reverse();
    r.reverse();
    let n = s.len();
    for i in 0..n {
        if let Some(&si) = s.last() {
            if si == '#' { s.pop();r.pop(); } else { break; }
        }
    }
    s.reverse();
    r.reverse();
    let n = s.len();

    // 各区間のmaxとその数を取得
    let mut max: Vec<usize> = Vec::new();
    let mut cnt: Vec<usize> = Vec::new();
    let mut now = ' ';
    for (i, &si) in s.iter().enumerate() {
        let ri = r[i];
        if now != si {
            max.push(r[i]);
            cnt.push(1);
        } else {
            let j = max.len()-1;
            if max[j] < ri {
                max[j] = ri;
                cnt[j] = 1;
            } else if max[j] == ri {
                cnt[j] += 1;
            }
        }
        now = si;
    }

    // 最大に関わる区間の組合せの数を取得
    let max_r= *r.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..(max.len()/2) {
        if max[2*i] == max_r || max[2*i+1] == max_r || max[2*i+2] == max_r {
            ans += cnt[2*i]*cnt[2*i+2];
        }
    }
    println!("{}", ans);
}