use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut tmp = 0;
        let mut i = i;
        while i > 0 {
            tmp += i%10;
            i /= 10;
        }
        if tmp == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}