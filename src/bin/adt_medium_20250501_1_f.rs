use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    fn f(n: usize) -> Vec<usize> {
        if n == 1 {
            return vec![1]
        }

        let mut ret = f(n-1);
        let tmp = ret.clone();
        ret.push(n);
        ret.extend(tmp);

        ret
    }

    let ans = f(n);
    println!("{}", ans.iter().join(" "));
}