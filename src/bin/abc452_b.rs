use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut ans: Vec<Vec<char>> = vec![Vec::new(); h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h-1 || j == 0 || j == w-1 {
                ans[i].push('#');
            } else {
                ans[i].push('.');
            }
        }
    }
    for a in ans.iter() {
        println!("{}", a.iter().join(""));
    }
}