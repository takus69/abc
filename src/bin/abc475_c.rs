use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: usize,
        l: usize,
        a: [usize; n-1],
    }
    s -= 1;
    let mut prefix: Vec<usize> = vec![0; n];
    for i in 0..(n-1) {
        prefix[i+1] = prefix[i] + a[i];
    }
    let mut ans = 1;
    for i in 0..n {
        if i <= s {
            let mut tmp = s-i+1;
            let len = (prefix[s] - prefix[i])*2;
            if len/2 <= l {
                ans = ans.max(s-i+1);
            }
            if l < len { continue; }
            let mut ok = s;
            let mut ng = n;
            while ok+1 < ng {
                let m = (ok + ng)/2;
                let len2 = prefix[m] - prefix[s];
                if len2+len <= l {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            tmp += ok - s;
            // println!("{} => {} => {}, {}+{}, {}", s, i, ok, len, prefix[ok]-prefix[s], tmp);
            ans = ans.max(tmp);
        }
        if i >= s {
            let mut tmp = i-s+1;
            let len = (prefix[i] - prefix[s])*2;
            if len/2 <= l {
                ans = ans.max(i-s+1);
            }
            if l < len { continue; }
            let mut ok = s as isize;
            let mut ng = -1;
            while ok > ng+1 {
                let m = (ok + ng)/2;
                let len2 = prefix[s] - prefix[m as usize];
                if len2+len <= l {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            tmp += s - ok as usize;
            // println!("{} => {} => {}, {}+{}, {}", s, i, ok, len, prefix[ok as usize]-prefix[s], tmp);
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}