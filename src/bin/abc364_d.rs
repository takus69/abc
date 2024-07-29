use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        bk: [(i64, usize); q],
    }
    a.sort();
    let min_a = a[0];
    let max_a = a[n-1];

    for (b, k) in bk.iter() {
        // bから0～一番離れているaの値で二分探索
        // println!("b: {}", b);
        let (mut l, mut r) = (-1, (max_a-b).abs().max((min_a-b).abs())+1);
        let mut ans = r;
        while l+1 < r {
            let x = (l + r) / 2;
            let cnt_r = a.partition_point(|d| d <= &(b+x));
            let cnt_l = a.partition_point(|d| d < &(b-x));
            // println!("k: {}, b+x: {}, b-x: {}, x: {}, l: {}, r: {}, cnt_r: {}, cnt_l: {}", k, b+x, b-x, x, l, r, cnt_r, cnt_l);
            if *k > cnt_r-cnt_l {
                l = x;
            } else {
                r = x;
                ans = x;
            }
        }
        println!("{}", ans);
    }
}