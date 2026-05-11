use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [[usize; n]; n],
        }
        let mut nim = 0;
        for i in 0..n {
            for j in 0..n {
                if (i+j) % 2 == 1 {
                    nim ^= (a[i][j])%(k+1);
                }
            }
        }
        if nim == 0 {
            println!("Bob");
        } else {
            println!("Alice");
        }
    }
}