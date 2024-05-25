use proconio::input;
use ac_library::{ModInt998244353, convolution};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [ModInt998244353; n],
        b: [ModInt998244353; m],
    }

    let ans = convolution(&a, &b);
    println!("{}", ans.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));

}