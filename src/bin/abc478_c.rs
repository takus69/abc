use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut b = a.clone();
    b.sort();
    let mut l = n-1;
    for i in 0..n {
        if a[i] > b[i] {
            l = i;
            break;
        }
    }
    let mut r = 0;
    for i in (0..n).rev() {
        if a[i] < b[i] {
            r = i;
            break;
        }
    }
    if k+l > r {
        println!("Yes");
    } else {
        println!("No");
    }
}