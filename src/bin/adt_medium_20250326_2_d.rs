use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut left = k;
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut ai = a[i];
        while left >= ai {
            left -= ai;
            i += 1;
            if i < n {
                ai = a[i];
            } else {
                break;
            }
        }
        ans += 1;
        left = k;
    }
    println!("{}", ans);
}