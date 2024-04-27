use proconio::{input, marker::Chars};

struct Coordinate {
    h: usize,
    w: usize,
}

impl Coordinate {
    fn add(&self, i: usize, j: usize, di: i32, dj: i32) -> Result<(usize, usize), &str> {
        if (i as i32) < -di || (j as i32) < -dj || (i as i32) + di == self.h as i32 || (j as i32) + dj == self.w as i32 {
            return Err("");
        }
        let i2 = (i as i32 + di) as usize;
        let j2 = (j as i32 + dj) as usize;
        Ok((i2, j2))
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    fn dfs(i: usize, j: usize, s: &Vec<Vec<char>>, v: &mut Vec<Vec<bool>>, r: &mut Vec<(usize, usize)>) -> i64 {
        // println!("dfs i, j: {} {}", i, j);
        let mut ret = 1;
        let h = s.len();
        let w = s[0].len();
        let cd = Coordinate{ h, w };
        let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for d in dir {
            let result = cd.add(i, j, d.0, d.1);
            if result.is_err() { continue; };
            let (di, dj) = result.unwrap();
            if s[di][dj] == '#' {
                r.push((i, j));
                return ret;
            }
        }

        for d in dir {
            let result = cd.add(i, j, d.0, d.1);
            if result.is_err() { continue; };
            let (di, dj) = result.unwrap();
            if s[di][dj] != '#' && !v[di][dj] {
                v[di][dj] = true;
                ret += dfs(di, dj, s, v, r);
                // println!("dfs di: {} dj: {}, ret: {}", di, dj, ret);
            }
        }
        ret
    }

    let mut visited = vec![vec![false; w]; h];
    let mut ans = 1;
    let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let cd = Coordinate{ h, w };

    for i in 0..h {
        for j in 0..w {
            let mut flg = false;
            // println!("start i: {} j: {}", i, j);
            for d in dir {
                let result = cd.add(i, j, d.0, d.1);
                if result.is_err() { continue; };
                let (di, dj) = result.unwrap();
                if s[di][dj] == '#' {
                    // println!("di: {} dj: {}", di, dj);
                    flg = true;
                    break;
                }
            }
            if flg { continue; }
            if s[i][j] != '#' && !visited[i][j] {
                // println!("dfs i: {} j: {}", i, j);
                let mut reset: Vec<(usize, usize)> = Vec::new();
                visited[i][j] = true;
                let tmp = dfs(i, j, &s, &mut visited, &mut reset);
                // println!("{} {} {}", i, j, tmp);
                ans = ans.max(tmp);
                for (i, j) in reset {
                    visited[i][j] = false;
                }
            }
        }
    }

    // println!("{:?}", visited);
    println!("{}", ans);
}