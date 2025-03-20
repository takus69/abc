use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut even = 0;
    let mut odd = 0;
    let mut even_cnt = 0;
    let mut odd_cnt = 0;
    for ai in a.iter().rev() {
        if ai % 2 == 0 {
            if even_cnt < 2 {
                even += ai;
                even_cnt += 1;
            }
        } else {
            if odd_cnt < 2 {
                odd += ai;
                odd_cnt += 1;
            }
        }
    }
    let mut ans = if even_cnt == 2 { even } else { 0 };
    ans = if odd_cnt == 2 { ans.max(odd) } else { ans };
    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}