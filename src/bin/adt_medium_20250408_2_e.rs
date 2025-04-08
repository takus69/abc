use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        mut a: [i64; n],
    }
    a.sort();
    if x < 0 {
        x *= -1;
    }
    let mut i = 0;
    let mut j = 1;
    while j < n {
        let diff = a[j] - a[i];
        if x == diff {
            println!("Yes");
            return;
        } else if x > diff || i == j {
            j += 1;
        } else {
            i += 1;
        }

    }
    println!("No");
}