use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let mut cal = 0;
    let mut eaten: Vec<bool> = vec![false; n];
    for (i, &ai) in a.iter().enumerate() {
        if i >= m && eaten[i-m] {
            cal -= a[i-m];
        }
        if cal+ai <= k {
            eaten[i] = true;
            cal += ai;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}