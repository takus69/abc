use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();
    let mut prefix_a: Vec<usize> = vec![0; n+1];
    for i in 0..n {
        prefix_a[i+1] = prefix_a[i] + a[i];
    }
    let mut prefix_b: Vec<usize> = vec![0; m+1];
    let mut cnt_b: Vec<usize> = vec![0; m+1];
    for j in 0..m {
        prefix_b[j+1] = prefix_b[j] + b[j];
        cnt_b[j+1] = cnt_b[j] + (b[j]-1)/k+1;
    }
    let mut ans = 0;
    for j in 0..=m {
        if cnt_b[j] > y { break; }
        let mut tmp = j;
        let mut money = y*k + x;
        money -= prefix_b[j];
        let i = match prefix_a.binary_search(&money) {
            Ok(i) => {i},
            Err(i) => {i-1},
        };
        tmp += i;
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}