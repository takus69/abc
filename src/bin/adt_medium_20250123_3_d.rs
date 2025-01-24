use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        let mut ans = 0;
        for ai in a.iter() {
            if ai%2 == 1 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}