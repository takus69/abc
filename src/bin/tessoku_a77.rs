use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }

    fn can_cut(a: &[usize], l: usize, m: usize, k: usize) -> bool {
        let mut cnt = 0;
        let mut pre = 0;
        let mut tmp = 0;
        for &ai in a {
            tmp = ai - pre;
            if ai - pre >= m {
                cnt += 1;
                pre = ai;
            }
        }
        
        cnt > k
    }

    a.push(l);
    let mut ok = 1;
    let mut ng = l;
    while ok + 1 < ng {
        let m = (ok + ng) / 2;

        if can_cut(&a, l, m, k) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}