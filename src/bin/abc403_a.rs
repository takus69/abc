use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for (i, &ai) in a.iter().enumerate() {
        if i % 2 == 0 {
            ans += ai;
        }
    }
    println!("{}", ans);
}