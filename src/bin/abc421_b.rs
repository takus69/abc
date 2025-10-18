use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let mut a = vec![x, y];

    fn f(x: usize) -> usize {
        let mut x: Vec<char> = x.to_string().chars().collect();
        x.reverse();
        x.iter().join("").parse::<usize>().unwrap()
    }

    for i in 2..10 {
        let x = a[i-2];
        let y = a[i-1];
        a.push(f(x+y));
    }
    println!("{}", a[9]);
}