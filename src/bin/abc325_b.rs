use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    let mut ans = 0;
    for h in 0..24 {
        let mut tmp = 0;
        for (w, x) in wx.iter() {
            if (h+x)%24 >= 9 && (h+x+1)%24 >= 9 && (h+x+1)%24 <= 18 {
                tmp += w;
            }
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}