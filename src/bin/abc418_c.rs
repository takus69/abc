use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        b: [usize; q],
    }
    a.sort();
    let mut sum_a: Vec<usize> = vec![0];
    let mut tmp = 0;
    for &ai in a.iter() {
        tmp += ai;
        sum_a.push(tmp);
    }
    for &bi in b.iter() {
        let i = a.partition_point(|&x| x < bi);
        // b未満の累積和+残りの種類*(b-1)+1
        let ans = sum_a[i]+(n-i)*(bi-1)+1;
        if ans > *sum_a.last().unwrap() {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}