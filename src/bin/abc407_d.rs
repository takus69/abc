use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    fn dfs(status: Vec<Vec<bool>>, ij: usize, a: &Vec<Vec<usize>>) -> usize {
        let h = status.len();
        let w = status[0].len();
        let i = ij / w;
        let j = ij % w;
        // (i, j)に
        // 置かない場合
        let mut ans: Vec<usize> = Vec::new();
        let status1= status.clone();
        let ans1 = if ij != h*w-1 {
            dfs(status1, ij+1, a)
        } else {
            calc(status1, a)
        };
        ans.push(ans1);
        // 横に置く場合
        let mut status2 = status.clone();
        if j < w-1 && !status2[i][j] && !status2[i][j+1] {
            status2[i][j] = true;
            status2[i][j+1] = true;

            let ans2 = if ij != h*w-1 {
                dfs(status2, ij+1, a)
            } else {
                calc(status2, a)
            };
            ans.push(ans2);
        }
        // 縦に置く場合
        let mut status3 = status.clone();
        if i < h-1 && !status3[i][j] && !status3[i+1][j] {
            status3[i][j] = true;
            status3[i+1][j] = true;

            let ans3 = if ij != h*w-1 {
                dfs(status3, ij+1, a)
            } else {
                calc(status3, a)
            };
            ans.push(ans3);
        }

        fn calc(status: Vec<Vec<bool>>, a: &Vec<Vec<usize>>) -> usize {
            let h = status.len();
            let w = status[0].len();
            let mut ans = 0;
            for i in 0..h {
                for j in 0..w {
                    if !status[i][j] {
                        ans ^= a[i][j];
                    }
                }
            }

            ans
        }

        *ans.iter().max().unwrap()
    }

    let status = vec![vec![false; w]; h];
    let ans = dfs(status, 0, &a);
    println!("{}", ans);
}