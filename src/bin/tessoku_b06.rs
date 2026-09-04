use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut prefix: Vec<usize> = vec![0; n+1];
    for i in 0..n {
        prefix[i+1] = prefix[i] + a[i];
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let m = r-l+1;
        let cnt = prefix[r]-prefix[l-1];
        if m-cnt < cnt {
            println!("win");
        } else if m-cnt > cnt {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}