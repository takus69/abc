use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
    }
    let mut l: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            li: usize,
        }
        input! {
            ll: [usize; li],
        }
        l.push(ll);
    }
    input! {
        c: [usize; n],
    }
    for (i, &ci) in c.iter().enumerate() {
        let li = l[i].len();
        if li*ci < k {
            k -= li*ci;
        } else {
            println!("{}", l[i][(k-1)%li]);
            return;
        }
    }
}