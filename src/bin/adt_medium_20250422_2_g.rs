use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let cnt_a = n/a;
    let sum_a = a*cnt_a*(cnt_a+1)/2;
    let cnt_b = n/b;
    let sum_b = b*cnt_b*(cnt_b+1)/2;
    let c = lcm(a, b);  // 最大公倍数
    let cnt_c = n/c;
    let sum_c = c*cnt_c*(cnt_c+1)/2;
    let sum = n*(n+1)/2;

    println!("{}", sum - sum_a - sum_b + sum_c);
}