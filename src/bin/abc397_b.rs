use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    let mut ans2 = 0;
    let mut cnt = 0;
    for &si in s.iter() {
        cnt += 1;
        if si == 'i' && cnt%2 != 1 {
            ans.push('o');
            ans.push(si);
            cnt += 1;
            ans2 += 1;
        } else if si == 'o' && cnt%2 != 0 {
            ans.push('i');
            ans.push(si);
            cnt += 1;
            ans2 += 1;
        } else {
            ans.push(si);
        }
    }
    if ans.len() % 2 != 0 { ans.push('o');ans2 += 1; }
    println!("{}", ans2);
}