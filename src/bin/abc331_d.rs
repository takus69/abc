use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
    }

    // 二次元累積和
    let mut area: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut cnt = 0;
            if i > 0 {
                cnt += area[i-1][j];
            }
            if j > 0 {
                cnt += area[i][j-1];
            }
            if i > 0 && j > 0 {
                cnt -= area[i-1][j-1];
            }
            if p[i][j] == 'B' {
                cnt += 1;
            }
            area[i][j] = cnt;
        }
    }
    // println!("{:?}", area);

    // クエリを順に処理
    for _ in 0..q {
        input! {
            query: [usize; 4],
        }
        let i1 = query[0];
        let j1 = query[1];
        let i2 = query[2];
        let j2 = query[3];
        /*
        let l_h = i2 - i1 + 1;
        let l_w = j2 - j1 + 1;
        let d1_h = (n-i1%n)%n;
        let d2_h = (i2+1)%n;
        let d1_w = (n-j1%n)%n;
        let d2_w = (j2+1)%n;
        println!("l_h: {}, d1_h: {}, d2_h: {}", l_h, d1_h, d2_h);
        let mut h_cnt = 0;
        if l_h >= d1_h + d2_h {
            h_cnt = (l_h - d1_h - d2_h)/n;
        }
        let mut w_cnt = 0;
        if l_w >= d1_w + d2_w {
            w_cnt = (l_w - d1_w - d2_w)/n;
        }*/

        fn area_cnt(i: usize, j: usize, area: &Vec<Vec<usize>>) -> usize {
            let h = area.len();
            let w = area[0].len();
            let mut ret = area[h-1][w-1] * ((i+1)/h) * ((j+1)/w);
            if (i+1)%h > 0 {
                ret += area[i%h][w-1] * ((j+1)/w);
            }
            if (j+1)%w > 0 {
                ret += area[h-1][j%w] * ((i+1)/h);
            }
            if (i+1)%h > 0 && (j+1)%w > 0 {
                ret += area[i%h][j%w];
            }

            ret
        }

        let mut ans = area_cnt(i2, j2, &area);
        if i1 > 0 && j1 > 0 {
            ans += area_cnt(i1-1, j1-1, &area);
        }
        if i1 > 0 {
            ans -= area_cnt(i1-1, j2, &area);
        }
        if j1 > 0 {
            ans -= area_cnt(i2, j1-1, &area);
        }
        // println!("h_cnt: {}, l_h: {}, d1_h: {}, d2_h: {}", h_cnt, l_h, d1_h, d2_h);
        // println!("w_cnt: {}, l_w: {}, d1_w: {}, d2_w: {}", w_cnt, l_w, d1_w, d2_w);

        /*
        fn area_cnt(start: (usize, usize), end: (usize, usize), area: &Vec<Vec<usize>>) -> usize {
            if start.0 == end.0 || start.1 == end.1 { return 0; }
            let mut ret = area[end.0-1][end.1-1];
            if start.0 > 0 && start.1 > 0 {
                ret += area[start.0-1][start.1-1];
            }
            if start.0 > 0 {
                ret -= area[start.0-1][end.1-1];
            }
            if start.1 > 0 {
                ret -= area[end.0-1][start.1-1];
            }

            ret
        }

        // 全体を覆う部分
        let mut ans: usize = h_cnt * w_cnt * area_cnt((0, 0), (n, n), &area);  // area[n-1][n-1];

        // 左側面
        ans += h_cnt * area_cnt((0, 0), (n, d2_w), &area);  // area[n-1][d2_w-1];
        // 下側面
        ans += w_cnt * area_cnt((0, 0), (d2_h, n), &area);  // area[d2_h-1][n-1];
        // 右下
        ans += area_cnt((0, 0), (d2_h, d2_w), &area);  // area[d2_h-1][d2_w-1];
        
        // 左側面
        ans += h_cnt * area_cnt((0, n-d1_w), (n, n), &area);  // (area[n-1][n-1]-area[n-1][n-1-d1_w]);
        // 上側面
        ans += w_cnt * area_cnt((n-d1_h, 0), (n, n), &area);  // (area[n-1][n-1]-area[n-1-d1_h][n-1]);
        // 左上
        ans += area_cnt((n-d1_h, n-d1_w), (n, n), &area);  // area[n-1][n-1] - area[n-1-d1_h][n-1-d1_w];  // ここ違う
        // 左下
        ans += area_cnt((0, n-d1_w), (d2_h, n), &area);
        // 右上
        ans += area_cnt((n-d1_h, 0), (n, d2_w), &area);
        */

        println!("{}", ans);
    }
}