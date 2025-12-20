use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();
    let mut cumsum_b: Vec<usize> = vec![0];
    let mut now_b = 0;
    for &bi in b.iter() {
        now_b += bi;
        cumsum_b.push(now_b);
    }

    let r#mod = 998244353;
    let mut ans = 0;
    for &ai in a.iter() {
        let i = b.partition_point(|&x| x < ai);
        ans += ai*i - cumsum_b[i];
        ans += cumsum_b[m]-cumsum_b[i] - ai*(m-i);
        ans %= r#mod;
    }

    println!("{}", ans);
}