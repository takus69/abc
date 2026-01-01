use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n],
    }
    let mut ans = 0;
    for &(ai, bi) in ab.iter() {
        if bi == 'W' {
            ans = ans.max(ai);
        } else {
            ans = ans.max(l-ai);
        }
    }
    println!("{}", ans);
}