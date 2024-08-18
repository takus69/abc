use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut sum_a: Vec<usize> = Vec::new();
    sum_a.push(0);
    let mut tmp = 0;
    for i in 1..n {
        if i%2 == 0 {
            tmp += a[i] - a[i-1];
        }
        sum_a.push(tmp);
    }
    // println!("sum_a: {:?}", sum_a);
    
    fn ans(l: usize, r: usize, a: &Vec<usize>, sum_a: &Vec<usize>) -> usize {
        // lの右のai(lai)と、rの左のai(rai)を探す(二分探索)
        // laiからraiまでの累積和で睡眠時間を求める
        // aiが奇数、偶数で動作が変わる
        // laiが奇数の場合は、laiからの累積和を求めるだけ。偶数の場合は、lai-lの睡眠時間を足して、laiからの累積和を求める
        // raiが奇数の場合は、r-raiの睡眠時間を足して、raiまでの累積和を求める
        let li = a.partition_point(|&x| x < l);
        let ri = a.partition_point(|&x| x <= r);
        if ri == 0 || li == a.len() { return 0; }
        let mut ans = sum_a[ri-1] - sum_a[li];
        // println!("li: {}, ri: {}, ans: {}", li, ri, ans);
        if li % 2 == 0 {
            ans += a[li] - l;
        }
        if ri % 2 == 0 {
            ans += r - a[ri-1];
        }
        ans
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", ans(l, r, &a, &sum_a));
    }
}