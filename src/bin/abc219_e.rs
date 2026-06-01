use proconio::input;

fn main() {
    input! {
        a: [[usize; 4]; 4],
    }
    let mut ans = 0;
    let mut used: Vec<Vec<bool>> = vec![vec![false; 5]; 5];
    let v: Vec<Vec<bool>> = vec![vec![false; 5]; 5];
    let h: Vec<Vec<bool>> = vec![vec![false; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            ans += dfs(i, j, i, j, &used, &v, &h, &a);
            used[i][j] = true;
        }
    }

    println!("{}", ans/2);

    fn check(v: &[Vec<bool>], h: &[Vec<bool>], a: &[Vec<usize>]) -> bool {
        for i in 0..4 {
            let mut v_cnt = 0;
            for j in 0..5 {
                if v[i][j] {
                    v_cnt += 1;
                }
                if j < 4 && a[i][j]==1 && v_cnt%2==0 {
                    return false;
                }
            }
            if v_cnt%2==1 {
                return false;
            }
        }
        true
    }

    fn dfs(si: usize, sj: usize, i: usize, j: usize, used: &Vec<Vec<bool>>, v: &Vec<Vec<bool>>, h: &Vec<Vec<bool>>, a: &[Vec<usize>]) -> usize {
        if (si, sj) == (i, j) && used[i][j] && check(&v, &h, a) {
            return 1;
        }
        let mut ret: usize = 0;
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || j2 < 0 || i2 > 4 || j2 > 4 { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if used[i2][j2] { continue; }
            let mut used2 = used.clone();
            used2[i2][j2] = true;
            let mut v2: Vec<Vec<bool>> = v.clone();
            let mut h2: Vec<Vec<bool>> = h.clone();
            if di != 0 {
                v2[i.min(i2)][j] = true;
            } else {
                h2[i][j.min(j2)] = true;
            }
            ret += dfs(si, sj, i2, j2, &used2, &v2, &h2, a);
        }

        ret
    }
}