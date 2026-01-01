use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a: Vec<usize> = Vec::new();
    for i in 1..=n {
        a.push(i);
    }
    let mut reverse = false;
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let i = if reverse { n-x } else { x-1 };
                a[i] = y;
            },
            2 => {
                reverse = !reverse;
            },
            3 => {
                input ! { x: usize, }
                let i = if reverse { n-x } else { x-1 };
                println!("{}", a[i]);
            },
            _ => {},
        }
    }
}