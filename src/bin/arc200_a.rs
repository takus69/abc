use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [isize; n],
            b: [isize; n],
        }
        let mut x: Vec<isize> = vec![0; n];
        let c = a[0] as f64 / b[0] as f64;
        let mut flg = false;
        for i in 1..n {
            let c2 = a[i] as f64 / b[i] as f64;
            if c == c2 {
                continue;
            } else if c > c2 {
                x[0] = a[i]+b[i];
                x[i] = -a[0]-b[0];
                flg = true;
                break;
            } else {
                x[0] = -a[i]-b[i];
                x[i] = a[0]+b[0];
                flg = true;
                break;
            }
        }
        if flg {
            println!("Yes");
            println!("{}", x.iter().join(" "));
        } else {
            println!("No");
        }
    }
}