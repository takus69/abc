use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(usize, usize); n],
    }
    let mut ans = 0;
    let mut pre_ti = 0;
    for (ti, vi) in tv.iter() {
        if ans > ti-pre_ti {
            ans -= ti-pre_ti;
        } else {
            ans = 0;
        }
        ans += vi;
        pre_ti = *ti;
    }
    println!("{}", ans);
}