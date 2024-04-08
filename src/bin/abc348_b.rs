use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    for i in 0..n {
        let mut ans = 0;
        let mut max_len = 0;
        for j in 0..n {
            if i == j { continue; }
            let l = (xy[i].0 - xy[j].0)*(xy[i].0 - xy[j].0) + (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            if max_len < l {
                max_len = l;
                ans = j+1;
            }
        }
        println!("{}", ans);
    }
}
