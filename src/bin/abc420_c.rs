use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += a[i].min(b[i]);
    }
    for _ in 0..q {
        input! {
            c: char,
            x: usize,
            v: usize,
        }
        let before = a[x-1].min(b[x-1]);
        if c=='A' {
            a[x-1] = v;
        } else {
            b[x-1] = v;
        }
        let after = a[x-1].min(b[x-1]);
        if before > after {
            ans -= before - after;
        } else {
            ans += after - before;
        }

        println!("{}", ans);
    }
}