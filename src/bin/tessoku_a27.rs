use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    fn gcd(a: usize, b: usize) -> usize {
        if a%b == 0 {
            b
        } else {
            gcd(b, a%b)
        }
    }

    println!("{}", gcd(a, b));
}