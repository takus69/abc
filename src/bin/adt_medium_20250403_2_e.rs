use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    // 桁を求める
    let mut n2 = n;
    let mut ans: Vec<usize> = Vec::new();
    for i in (0..=18).rev() {
        let m = 5_usize.pow(i);
        for k in (1..=5).rev() {
            if k*m < n2 {
                n2 -= k*m;
                ans.push(k);
                break;
            }
            if k == 1 {
                ans.push(0);
            }
        }
    }
    ans.reverse();

    // 別解(解説)
    let mut ans: Vec<usize> = Vec::new();
    let mut n2 = n-1;
    while n2 > 0 {
        ans.push(n2%5);
        n2 /= 5;
    }

    let mut ans2 = 0;
    let v = [0, 2, 4, 6, 8];
    for (i, &k) in ans.iter().enumerate() {
        ans2 += v[k] * 10_usize.pow(i as u32);
    }
    println!("{}", ans2);

}