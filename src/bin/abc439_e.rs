use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by(|a, b| a.0.max(a.1).cmp(&(b.0.max(b.1))));
    // println!("ab: {:?}", ab);
    let mut dp: Vec<Vec<usize>> = vec![vec![0, 0]];
    let mut pre_a = (0, 0);
    let mut pre_b = (0, 0);
    for &(a, b) in ab.iter() {
        let dpi = dp.last().unwrap();
        let (mut dp0, mut dp1) = (dpi[0], dpi[1]);
        let dp2 = (dp0+1).max(dp1+1);
        if pre_a.0 < a && pre_a.1 < b {
            if pre_b.0 < a && pre_b.1 < b {
                dp0 = dp2;
            } else {
                dp0 += 1;
            }
            pre_a = (a, b);
        } else if pre_a.0 > a {
            pre_a = (a, b);
        }
        if pre_b.0 < a && pre_b.1 < b {
            if pre_a.0 < a && pre_a.1 < b {
                dp1 = dp2;
            } else {
                dp1 += 1;
            }
            pre_b = (a, b);
        } else if pre_b.1 > b {
            pre_b = (a, b);
        }
        dp.push(vec![dp0, dp1]);
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}