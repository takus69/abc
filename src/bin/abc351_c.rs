use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans: VecDeque<usize> = VecDeque::new();
    for ai in a {
        ans.push_back(ai);
        let mut l = ans.len();
        if l > 1 {
            while ans[l-1] == ans[l-2] {
                let q = ans.pop_back().unwrap();
                ans.pop_back();
                ans.push_back(q+1);
                l -= 1;
                if l == 1 { break; }
            }
        }
    }
    println!("{}", ans.len());
}