use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = String::new();
    for i in (0..=60).rev() {
        if (n>>i)&1==1 {
            if !ans.is_empty() {
                ans.push('B');
            }
            ans.push('A');
        } else if !ans.is_empty() {
            ans.push('B');
        }
    }

    println!("{}", ans);
}