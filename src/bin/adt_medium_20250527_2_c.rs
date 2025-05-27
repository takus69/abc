use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    let mut ans = [0; 24];
    for &(wi, xi) in wx.iter() {
        for i in 0..24 {
            let t = (i+xi) % 24;
            if t >= 9 && t < 18 {
                ans[i] += wi;
            }
        }
    }
    println!("{}", ans.iter().max().unwrap());
}