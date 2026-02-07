use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.push(0);
    a.sort();
    a.reverse();

    let mut cnt: Vec<usize> = Vec::new();
    let mut now = 0;
    for w in a.windows(2) {
        now += 1;
        let pre_ai = w[0];
        let ai = w[1];
        for _ in 0..(pre_ai-ai) {
            cnt.push(now);
        }
    }

    cnt.reverse();
    let mut ans: Vec<usize> = vec![cnt[0]];
    let m = cnt.len();
    for i in 0..(m-1) {
        let ci = ans[i];
        ans[i] = ci%10;
        ans.push(cnt[i+1] + ci / 10) ;
    }
    
    ans.reverse();
    println!("{}", ans.iter().join(""));
}