use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }

    fn f(n: usize) -> Vec<Vec<char>> {
        if n == 0 {
            return vec![vec!['#']];
        }

        let tmp = f(n-1);
        let m = 3_usize.pow((n-1) as u32);
        let mut tmp2 = tmp.clone();
        for i in 0..m {
            tmp2[i].extend(&tmp[i]);
            tmp2[i].extend(&tmp[i]);
        }
        let mut ans = tmp2.clone();
        for i in 0..m {
            ans.push(tmp[i].clone());
            ans[i+m].extend(vec!['.'; m]);
            ans[i+m].extend(tmp[i].clone());
        }
        ans.extend(tmp2);

        ans
    }

    let ans = f(n);
    for a in ans.iter() {
        println!("{}", a.iter().join(""));
    }
}