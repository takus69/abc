use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    }
    // wを二分探索(logN)
    // wの時にM行に収まるか(M)
    // wに収まるlの位置を二分探索で定める(logN)
    // l(ng): M行超, r(ok): M行以下で収まる
    let mut sum_l: Vec<usize> = vec![0];
    for li in l.iter() {
        sum_l.push(sum_l.last().unwrap() + li + 1);
    }

    let mut ng: usize = 0;
    let mut ok: usize = l.iter().sum::<usize>() + n;
    // println!("{:?}", sum_l);
    let mut j = 0;
    while ng +1 < ok {
        let w = (ng + ok) / 2;
        // println!("w: {}, ng: {}, ok: {}", w, ng, ok);
        let mut cnt = 0;
        let mut pre = 0;
        for i in 0..m {
            j = sum_l.partition_point(|&x| x <= pre+w+1);
            pre = sum_l[j-1];
            cnt += 1;
            // println!("i行目: {}, 長さ: {}, チェック: {}, cnt: {}", i+1, sum_l[j-1], (w+1)*(i+1), cnt);
            if j == n+1 {
                break;
            }
        }
        // println!("j: {}", j);
        if j < n+1 {
            ng = w;
        } else {
            ok = w;
        }
    }
    println!("{}", ok);
}