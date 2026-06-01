use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        a: Chars,
        b: Chars,
    }
    
    fn to_decimal(s: &[char], k: usize) -> usize {
        let mut res = 0;
        for c in s {
            res = res*k + c.to_digit(10).unwrap() as usize;
        }

        res
    }

    let a2 = to_decimal(&a, k);
    let b2 = to_decimal(&b, k);
    println!("{}", a2*b2);
}