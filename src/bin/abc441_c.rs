use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = 0;
    for _ in 0..(n-k) {
        a.pop();
        ans += 1;
    }
    let mut x2 = 0;
    while let Some(ai) = a.pop() {
        x2 += ai;
        ans += 1;
        if x2 >= x { break; }
    }
    if x2 >= x {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}