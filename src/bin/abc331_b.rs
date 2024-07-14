use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    }
    let l_cnt = (n-1) / 12 + 1;
    let m_cnt = (n-1) / 8 + 1;
    let s_cnt = (n-1) / 6 + 1;
    
    let mut ans = usize::MAX;
    for li in 0..=l_cnt {
        for mi in 0..=m_cnt {
            for si in 0..=s_cnt {
                if 12*li + 8*mi + 6*si < n { continue; }
                let price = li*l + mi*m + si*s;
                ans = ans.min(price);
            }
        }
    }
    println!("{}", ans);
}