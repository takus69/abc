use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }
    let l_sum: usize = l.iter().sum();
    let mut tmp = 0;
    let mut ans = l_sum;
    for &li in &l {
        tmp += li;
        ans = ans.min(tmp.abs_diff(l_sum-tmp));
    }
    println!("{}", ans);
}