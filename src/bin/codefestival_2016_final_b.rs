use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans: Vec<i64> = Vec::new();
    let mut sum_ans: i64 = 0;
    let mut remove: i64 = 0;
    for i in 1..=n {
        ans.push(i);
        sum_ans += i;
        if sum_ans >= n {
            remove = sum_ans - n;
            break;
        }
    }

    for a in ans {
        if a != remove {
            println!("{}", a);
        }
    }

}
