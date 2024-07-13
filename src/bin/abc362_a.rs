use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }
    let mut ans = r.min(g);
    if c == "Red" {
        ans = g.min(b);
    } else if c == "Green" {
        ans = b.min(r);
    }
    println!("{}", ans);
}