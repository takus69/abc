use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
        c: [isize; n],
    }

    let mut d1 = a[0]+a[1];
    let mut d2 = a[0]+b[1];
    let mut d3 = 0;
    for i in 2..n {
        d3 = d3.max(d2) + c[i];
        d2 = d2.max(d1) + b[i];
        if i < n-2 {
            d1 += a[i];
        }
    }

    println!("{}", d3);
}