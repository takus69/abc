use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut stack: Vec<usize> = vec![];  // 題意よりstackは単調減少(逆に入れている)(小さいのは削除して、大きいのを残す)
    let mut ans: Vec<usize> = vec![];
    for i in (0..n).rev() {
        ans.push(stack.len());
        let hi = h[i];
        while !stack.is_empty() && stack.last().unwrap() < &hi {
            stack.pop();
        }

        if stack.is_empty() || stack.last().unwrap() > &hi {
            stack.push(hi);
        }
    }

    ans.reverse();
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}