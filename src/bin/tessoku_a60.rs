use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans: Vec<isize> = Vec::new();
    let mut stock: Vec<(usize, usize)> = Vec::new();
    for (i, &ai) in a.iter().enumerate() {
        while let Some(&(v, _)) = stock.last() {
            if v < ai {
                stock.pop();
            } else {
                break;
            }
        }
        match stock.last() {
            None => {
                ans.push(-1);
            },
            Some(&(_, j)) => {
                ans.push((j+1) as isize);
            }
        }
        stock.push((ai, i));
    }
    println!("{}", ans.iter().join(" "));
}