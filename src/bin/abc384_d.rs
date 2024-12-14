use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: isize,
        mut a: [isize; n],
    }
    let mut ans = "No";

    let sum_a: isize = a.iter().sum();
    s %= sum_a;
    if s == 0 {
        ans = "Yes";
    }
    // 尺取り
    a.extend(a.clone());
    let mut l = 0;
    let mut tmp = a[l];
    let mut r = 1;
    while tmp > 0 || r < n*2 {
        if tmp == s {
            ans = "Yes";
            break;
        } else if tmp > s {
            tmp -= a[l];
            l += 1;
        } else {
            if r < n*2 {
                tmp += a[r];
                r += 1;
            }
            if r >= n*2 && tmp < s { break; }
        }
    }

    println!("{}", ans);
}