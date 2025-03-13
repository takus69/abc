use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ans = 0;
    let mut l = 0;
    let mut r = 1;
    for _ in 0..q {
        input! {
            h: char,
            mut t: usize,
        }
        t -= 1;
        let mut tmp = 0;
        let mut tmp2 = 0;
        let mut l2 = l;
        let mut r2 = r;
        if h == 'L' {
            while l != t {
                l += 1;
                l %= n;
                tmp += 1;
                if l == r {
                    tmp = usize::MAX;
                    break;
                }
            }
            while l2 != t {
                l2 += n-1;
                l2 %= n;
                tmp2 += 1;
                if l2 == r2 {
                    tmp2 = usize::MAX;
                    break;
                }
            }
        } else {
            while r != t {
                r += 1;
                r %= n;
                tmp += 1;
                if l == r {
                    tmp = usize::MAX;
                    break;
                }
            }
            while r2 != t {
                r2 += n-1;
                r2 %= n;
                tmp2 += 1;
                if l2 == r2 {
                    tmp2 = usize::MAX;
                    break;
                }
            }
        }
        if tmp < tmp2 {
            ans += tmp;
        } else {
            ans += tmp2;
            l = l2;
            r = r2;
        }
    }
    println!("{}", ans);
}