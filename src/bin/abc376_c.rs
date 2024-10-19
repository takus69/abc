use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n-1],
    }
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();

    let mut ans = -1;
    let mut j = 0;
    let mut x = 0;
    for i in 0..n {
        let ai = a[i];
        let bi = if j < n-1 { b[j] } else { 0 };
        if ai <= bi {
            j += 1;
        } else {
            if x == 0 {
                x = a[i];
            } else {
                x = i64::MAX;
                break;
            }
        }
    }
    if x != i64::MAX {
        ans = x;
    }
    println!("{}", ans);
}