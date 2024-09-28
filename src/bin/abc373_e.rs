use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let mut sorted = a.clone();
    sorted.sort();
    sorted.reverse();
    let mut sum_a: Vec<usize> = vec![0];
    for i in 0..n {
        sum_a.push(sum_a[i] + sorted[i]);
    }
    let left = k - sum_a.last().unwrap();
    
    let mut ans: Vec<i64> = vec![-1; n];
    for (j, ai) in a.iter().enumerate() {
        if n == m { ans[j] = 0; continue; }
        // println!("j: {}, ai: {}", j, ai);
        // 全部aiに投票してOKか？
        let mut xi = ai+left;
        let i = sorted.partition_point(|&x| x > xi);  // xiより大きい人がi人いる
        if i >= m {
            ans[j] = -1;
            continue;
        }
        // aiに投票せずNGか？
        let i = sorted.partition_point(|x| x > ai);
        let d = if i <= m { 1 } else { 0 };
        let mut xi = *ai;
        let i = sorted.partition_point(|&x| x > xi);  // xiより大きい人がi人いる
        if i >= m || sum_a[m+d]+left < sum_a[i]+(xi-ai)*(1-d)+xi*d || (xi+1)*(m-i) <= sum_a[m+d]+left-sum_a[i]-(xi-ai)*(1-d)-xi*d {
        } else {
            ans[j] = 0;
            continue;
        }
        
        let mut ng = *ai;
        let mut ok = ai+left;
        let i = sorted.partition_point(|x| x > ai);
        let d = if i <= m { 1 } else { 0 };
        while ng+1 < ok {
            xi = (ng + ok) / 2;
            let i = sorted.partition_point(|&x| x > xi);  // xiより大きい人がi人いる
            // println!("xi: {}, ng: {}, ok: {}, sum: {}, {}, sum_a[m+1]: {}, sum_a[i]: {}, left: {}", xi, ng, ok, (xi+d)*(m-i), sum_a[m+d]+left-sum_a[i]-xi, sum_a[m+d], sum_a[i], left);
            if i >= m || sum_a[m+d]+left < sum_a[i]+(xi-ai)*(1-d)+xi*d || (xi+1)*(m-i) <= sum_a[m+d]+left-sum_a[i]-(xi-ai)*(1-d)-xi*d {
                ng = xi;
            } else {
                ok = xi;
            }
        }
        // println!("ans: {}, ok: {}, ng: {}", ok - ai, ok, ng);
        ans[j] = (ok - ai) as i64;
    }

    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}