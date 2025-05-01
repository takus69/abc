use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        mut n: usize,
    }
    if n == 1 {
        println!("0");
        return;
    } else {
        n -= 1;
    }
    for d in 1..36 {
        let x = (d+1)/2;
        let cnt = 9*10_usize.pow(x-1);
        if n <= cnt {
            let k = cnt - n + 1;
            let ans = 10_usize.pow(x) - k;
            let mut ans: Vec<char> = ans.to_string().chars().collect();
            let mut ans2 = if d%2==1 {
                ans[0..(ans.len()-1)].to_vec()
            } else {
                ans[0..].to_vec()
            };
            ans2.reverse();
            ans.extend(ans2);
            println!("{}", ans.iter().join(""));
            return;
        } else {
            n -= cnt;
        }
    }
}