use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 0;
    for si in s.iter() {
        if si == "Takahashi" {
            ans += 1;
        }
    }
    println!("{}", ans);
}