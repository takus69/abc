use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        apbq: [(usize, usize, usize, usize); n],
    }

    let mut ok= 0;
    let mut ng= 10_000_000_000;

    while ok+1 < ng {
        let mid = (ok + ng) / 2;
        if can_achieve(mid, n, x, &apbq) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn can_achieve(w: usize, n: usize, x: usize, apbq: &[(usize, usize, usize, usize)]) -> bool {
    let mut cost = 0;
    for i in 0..n {
        let (a, p, b, q) = apbq[i];
        let mut opt_cost = ((w-1)/a+1)*p;
        for j in 0..=b {
            if w < j*a+1 { break; }
            let tmp_cost = ((w-j*a-1)/b+1)*q + j*p;
            opt_cost = opt_cost.min(tmp_cost);
        }
        for j in 0..=a {
            if w < j*b+1 { break; }
            let tmp_cost = ((w-j*b-1)/a+1)*p + j*q;
            opt_cost = opt_cost.min(tmp_cost);
        }
        cost += opt_cost;
    }

    cost <= x
}
