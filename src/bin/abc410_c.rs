use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a: Vec<usize> = (1..=n).collect();
    let mut i = 0;
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                let ii = (p+i-1)%n;
                a[ii] = x;
            },
            2 => {
                input! {
                    p: usize,
                }
                let ii = (p+i-1)%n;
                println!("{}", a[ii]);
            },
            3 => {
                input! {
                    k: usize,
                }
                i += k;
                i %= n;
            },
            _ => {},
        }
    }
}