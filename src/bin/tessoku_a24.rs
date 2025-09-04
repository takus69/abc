use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut l: Vec<usize> = vec![usize::MAX; n+1];  // 要素数i個の最後の最小の値
    l[0] = 0;
    for &ai in a.iter() {
        let j = l.partition_point(|&x| x < ai);
        l[j] = l[j].min(ai);
        ans = ans.max(j);
    }
    println!("{}", ans);
}