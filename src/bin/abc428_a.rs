use proconio::input;

fn main() {
    input! {
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }
    let mut ans = 0;
    let mut time = 0;
    loop {
        if time + a <= x {
            ans += a * s;
        } else {
            ans += (x-time) * s;
            break;
        }
        time += a+b;
        if time >= x { break; }
    }
    println!("{}", ans);
}