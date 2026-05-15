use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut a2: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        a2[i].push(a[i]-1);
    }

    // 事前計算ダブリング
    let mut bucket: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        bucket[i].push(i+1);
    }
    for last_j in 0..=30 {
        for i in 0..n {
            // バケツを持っている人
            let last_i = a2[i][last_j];
            let w_ai = a2[last_i][last_j];
            a2[i].push(w_ai);

            // バケツに水を入れる
            let now = bucket[i][last_j];
            let add = bucket[last_i][last_j];
            bucket[i].push(now+add);
        }
    }

    for _ in 0..q {
        input! {
            t: usize,
            b: usize,
        }
        let mut ans = 0;
        let mut now = b-1;
        for i in (0..=30).rev() {
            if (t >> i)&1 == 1 {
                ans += bucket[now][i];
                now = a2[now][i];
            }
        }
        println!("{}", ans);
    }
}