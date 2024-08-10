use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut max_m = 0;
    for si in s.iter() {
        max_m = max_m.max(si.len());
    }
    let mut ans: Vec<Vec<char>> = vec![vec![]; max_m];
    for i in (0..n).rev() {
        for j in 0..max_m {
            if j < s[i].len() {
                ans[j].push(s[i][j]);
            } else {
                ans[j].push('*');
            }
        }
    }
    for i in 0..ans.len() {
        while ans[i][ans[i].len()-1] == '*' {
            ans[i].pop();
        }
    }
    for a in ans.iter() {
        println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}