use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    fn f(k: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(&n) = memo.get(&k) { n } else {
            let a = f(k/2, memo);
            let b = f(k/3, memo);
            memo.insert(k/2, a);
            memo.insert(k/3, b);
            a + b
        }
    }
    let mut memo: HashMap<usize, usize> = HashMap::new();
    memo.insert(0, 1);
    println!("{}", f(n, &mut memo));
}