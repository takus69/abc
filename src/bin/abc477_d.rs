use proconio::input;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ans: Vec<char> = vec!['a'; n];
    let mut tiles: Vec<bool> = vec![false; n];
    let mut stock: HashSet<usize> = HashSet::new();
    for i in 0..n {
        stock.insert(i);
    }
    let mut last_color: char = 'a';
    for _ in 0..q {
        input! {
            cmd: usize,
        }
        if cmd == 1 {
            input! {
                x: usize,
            }
            if !tiles[x-1] {
                stock.remove(&(x-1));
                if ans[x-1] == ' ' {
                    ans[x-1] = last_color;
                }
            } else {
                stock.insert(x-1);
            }
            tiles[x-1] = !tiles[x-1];
        } else {
            input! {
                c: char,
            }
            last_color = c;
            for &i in stock.iter() {
                ans[i] = ' ';
            }
            stock = HashSet::new();
        }
        // println!("ans: {:?}, tiles: {:?}, last_color: {}", ans, tiles, last_color);
    }

    for i in 0..n {
        if ans[i] == ' ' {
            ans[i] = last_color;
        }
    }
    println!("{}", ans.iter().join(""));
}