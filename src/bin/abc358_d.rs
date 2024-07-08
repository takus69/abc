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

    let mut ans = 0;
    let mut cnt = 0;
    let mut j = 0;
    for ai in a.iter() {
        let bj = &b[j];
        if ai >= bj {
            ans += ai;
            cnt += 1;
            j += 1;
        }
        if cnt == m { break; }
    }
    if cnt == m {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}