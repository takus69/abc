use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut a: [usize; n],
    }
    a.push(t);
    let mut ans = 0;
    let mut time = 0;
    for &ai in a.iter() {
        if time > ai {
            continue;
        }
        ans += ai - time;
        time = ai + 100;
    }
    println!("{}", ans);
}