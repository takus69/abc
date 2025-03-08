use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [isize; n],
        mut w: [isize; m],
    }
    b.sort();
    b.reverse();
    w.sort();
    w.reverse();
    let mut ans = 0;
    let mut tmp_ans = 0;
    let mut j = 0;
    for (i, &bi) in b.iter().enumerate() {
        tmp_ans += bi;
        ans = tmp_ans.max(ans);
        while j < m && j <= i && w[j] >= 0 {
            tmp_ans += w[j];
            j += 1;
            ans = tmp_ans.max(ans);
        }
    }
    println!("{}", ans);
}