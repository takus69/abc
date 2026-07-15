use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }
        let l = n.isqrt();
        let mut ans = l*(l-1)*2;
        let m = n-(l*l);
        // println!("l: {}, m: {}", l, m);
        if m > l {
            ans += m;
            ans += m-2;
        } else if m > 0 {
            ans += m;
            ans += m-1;
        }
        println!("{}", ans);
    }
}