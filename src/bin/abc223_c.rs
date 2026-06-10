use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    const t: f64 = 0.000001;

    // x cm以下の位置、時間、速度から、x cmの時間を算出
    fn search(x: f64, vec: &[(f64, f64, f64)]) -> f64 {
        // 二分探索で現時点以下の位置、時間、速度を探索
        let mut ok = 0;
        let mut ng = vec.len();
        while ok + 1 < ng {
            let m = (ok+ng)/2;
            if x <= vec[m].0 {
                ng = m;
            } else {
                ok = m;
            }
        }

        let (c, s, v) = vec[ok];
        s + if v > 0.0 { (x-c)/v } else { 0.0 }
    }

    // 累積和(c, s, v), c cmのときs秒で速度v
    let mut left: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0)];
    for &(a, b) in &ab {
        let last = left.len()-1;
        let a = a as f64;
        let b = b as f64;
        let (pre_a, pre_s, _) = left[last];
        left[last].2 = b;
        left.push((pre_a+a, pre_s+a/b, 0.0));
    }
    let mut right: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0)];
    for &(a, b) in ab.iter().rev() {
        let last = right.len()-1;
        let a = a as f64;
        let b = b as f64;
        let (pre_a, pre_s, _) = right[last];
        right[last].2 = b;
        right.push((pre_a+a, pre_s+a/b, 0.0));
    }

    // m cmのとき左の方が早く到着
    let mut ok = 0.0;
    let length = left.last().unwrap().0;
    let mut ng = length;
    while ok + t < ng {
        let m = (ok + ng)/2.0;
        let l_sec = search(m, &left);
        let r_sec = search(length-m, &right);
        if l_sec > r_sec {
            ng = m;
        } else {
            ok = m;
        }
    }
    println!("{}", ok);
}