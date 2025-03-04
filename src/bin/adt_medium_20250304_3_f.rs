use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();
    let mut ai = 0;
    let mut bi = 0;
    let mut ans = usize::MAX;
    while ai < a.len() && bi < b.len() {
        let ai2 = a[ai];
        let bi2 = b[bi];
        if ai2 > bi2 {
            ans = ans.min(ai2 - bi2);
            bi += 1;
        } else {
            ans = ans.min(bi2 - ai2);
            ai += 1;
        }
    }

    println!("{}", ans);
}