use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    }
    let mut ans = 0;
    for &ai in a.iter() {
        ans += ai*m;
    }
    for &ci in c.iter() {
        ans += ci*n;
    }
    ans += b*n*m;
    println!("{}", ans);
}