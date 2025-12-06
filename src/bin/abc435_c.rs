use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = a[0];
    for (i, &ai) in a.iter().enumerate() {
        if i < ans {
            ans = ans.max(i+ai);
            ans = ans.min(n);
        }
    }
    println!("{}", ans);
}