use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    b.sort();
    a.reverse();
    b.reverse();
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        // println!("ai: {}, bj: {}", a[i], b[j]);
        if (a[i] - b[j]).abs() <= d {
            println!("{}", a[i] + b[j]);
            std::process::exit(0);
        } else {
            if a[i] < b[j] {
                j += 1;
            } else {
                i += 1;
            }
            if i == n {
                i -= 1;
                j += 1;
            }
            if j == m {
                i += 1;
                j -= 1;
            }
        }
        // println!("i: {}, j: {}", i, j);
    }
    println!("-1");
}