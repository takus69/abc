use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 1;
    for ai in a.iter() {
        if ans.to_string().len() + ai.to_string().len() - 1 > k {
        ans = 1;
        continue;
        }
        ans *= ai;
        if ans.to_string().len() > k {
        ans = 1;
        }
    }
    println!("{}", ans);
}