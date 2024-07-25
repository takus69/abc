use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    }
    f.sort();
    f.reverse();
    let mut tmp = 0;
    let mut sum: Vec<usize> = Vec::new();
    for i in 0..n {
        if i % d == 0 {
            if tmp > 0 {
                sum.push(tmp);
            }
            tmp = 0;
        }
        tmp += f[i];
    }
    if tmp > 0 {
        sum.push(tmp);
    }
    let mut ans = 0;
    for si in sum.iter() {
        if si >= &p {
            ans += p;
        } else {
            ans += si;
        }
    }
    println!("{}", ans);
}