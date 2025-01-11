use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ok = 0;
    let mut ng = n/2+1;
    while ok+1 < ng {
        let mut flg = true;
        let k = (ok + ng) / 2;
        for i in 0..k {
            if a[i] > a[n-k+i]/2 {
                flg = false;
                break;
            }
        }
        if flg {
            ok = k;
        } else {
            ng = k;
        }
    }
    println!("{}", ok);
}