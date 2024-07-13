use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }
    let mut sum_l = 0;
    let mut sum_r = 0;
    for (l, r) in lr.iter() {
        sum_l += l;
        sum_r += r;
    }
    if sum_l <= 0 && sum_r >= 0 {
        println!("Yes");
    } else {
        println!("No");
        std::process::exit(0);
    }
    let mut ans: Vec<i64> = Vec::new();
    for (l, r) in lr.iter() {
        if sum_r == 0 {
            ans.push(*r);
        } else if r - l >= sum_r {
            ans.push(r - sum_r);
            sum_r = 0;
        } else {
            ans.push(*l);
            sum_r -= r - l;
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}