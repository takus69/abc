use proconio::input;

fn main() {
    input! {
        mut n: usize,
        r: usize,
        l: [usize; n],
    }
    let mut li = usize::MAX;
    let mut ri = usize::MAX;
    let mut l_cnt = 0;
    let mut r_cnt = 0;
    for i in r..n {
        if l[i] == 1 {
            ri = i;
            r_cnt += 1;
        } 
    }
    for i in (0..r).rev() {
        if l[i] == 1 {
            li = i;
            l_cnt += 1;
        }
    }
    if ri == n-1 {
        for i in (r..n).rev() {
            if l[i] == 1 {
                r_cnt -= 1;
                n -= 1;
            } else {
                break;
            }
        }
    }
    if li == 0 {
        for i in (0..r) {
            if l[i] == 1 {
                l_cnt -= 1;
                n -= 1;
            } else {
                break;
            }
        }
    }
    println!("{}", n+r_cnt+l_cnt);
}