use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ok = 0;
    let mut ng = usize::MAX/k;
    while ok+1 < ng {
        let p = (ok+ng)/2;
        let mut sum = 0;
        for &ai in &a {
            sum += p.min(ai);
        }
        if sum < p*k {
            ng = p;
        } else {
            ok = p;
        }
    }

    println!("{}", ok);
}