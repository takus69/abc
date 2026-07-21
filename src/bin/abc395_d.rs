use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut l2n: Vec<usize> = (0..=n).collect();
    let mut pigeon: Vec<usize> = (0..=n).collect();
    let mut n2l: Vec<usize> = (0..=n).collect();

    for _ in 0..q {
        input! {
            op: usize,
        }
        if op == 3 {
            input! {
                a: usize,
            }
            println!("{}", l2n[pigeon[a]]);
        } else {
            input! {
                a: usize,
                b: usize,
            }
            if op == 1 {
                pigeon[a] = n2l[b];
            } else {
                l2n.swap(n2l[a], n2l[b]);
                n2l.swap(a, b);
            }
        }
    }
}