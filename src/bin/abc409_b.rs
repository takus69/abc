use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = 0;
    for x in 0..=n {
        let mut cnt = 0;
        for &ai in a.iter() {
            if ai >= x {
                cnt += 1;
            }
        }
        if x <= cnt {
            ans = x;
        }
    }
    println!("{}", ans);
}