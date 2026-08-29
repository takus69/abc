use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans: Vec<Vec<usize>> = vec![Vec::new()];
    dfs(n, k, &mut ans, 0);
    for i in 0..ans.len() {
        ans[i].reverse();
    }
    ans.sort();
    for a in &ans {
        println!("{}", a.iter().join(" "));
    }

    fn dfs(n: usize, k: usize, ans: &mut Vec<Vec<usize>>, i: usize) {
        if n==1 {
            ans[i].push(k);
            return;
        }
        let max = k/n;
        let base_ans = ans[i].clone();
        ans[i].push(0);
        dfs(n-1, k, ans, i);
        for cnt in 1..=max {
            let mut base_ans2 = base_ans.clone();
            base_ans2.push(cnt);
            ans.push(base_ans2);
            dfs(n-1, k-cnt*n, ans, ans.len()-1);
        }
    }
}