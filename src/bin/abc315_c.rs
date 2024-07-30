use proconio::input;

fn main() {
    input! {
        n: usize,
        mut fs: [(usize, usize); n],
    }
    fs.sort_by(|a, b| b.1.cmp(&a.1));
    let mut ans = fs[0].1;
    let max_f = fs[0].0;
    let max_s = fs[0].1;
    for i in 1..fs.len() {
        let (fi, si) = (fs[i].0, fs[i].1);
        if max_f != fi {
            ans = ans.max(max_s + si);
        } else {
            ans = ans.max(max_s + si/2);
        }
    }
    println!("{}", ans);
}