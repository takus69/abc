use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut hato: Vec<usize> = (0..(n+1)).collect();
    let mut a2b: Vec<usize> = (0..(n+1)).collect();
    let mut b2a: Vec<usize> = (0..(n+1)).collect();
    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                hato[a] = b2a[b];
            },
            2 => {
                input! {
                    a: usize,
                    b: usize,
                }
                a2b[b2a[a]] = b;
                a2b[b2a[b]] = a;
                b2a.swap(a, b);
            },
            3 => {
                input! {
                    a: usize,
                }
                println!("{}", a2b[hato[a]]);
            },
            _ => {},
        }
    }
}