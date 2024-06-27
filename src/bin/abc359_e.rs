use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = Vec::new();
    ans.push(0);
    let mut data = VecDeque::new();
    data.push_back((0, 0));
    for (i, hi) in h.into_iter().enumerate() {
        let mut w = 1;
        while !data.is_empty() && data[data.len()-1].1 <= hi {
            let (w2, _) = data.pop_back().unwrap();
            w += w2;
        }
        data.push_back((w, hi));
        ans.push(ans[i+1-w] + w*hi);
    }
    let mut ans2 = Vec::new();
    for a in ans.iter() {
        ans2.push(a+1);
    }
    println!("{}", ans2[1..].iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
}