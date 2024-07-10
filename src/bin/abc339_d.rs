use proconio::{input, marker::Chars};
use std::collections::{VecDeque, HashMap};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut map: Vec<String> = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            map.push(s[i][j].to_string());
            if s[i][j] == 'P' {
                if cnt == 0 {
                    p1 = n*i + j;
                    cnt += 1;
                } else {
                    p2 = n*i + j;
                    cnt += 1;
                }
            }
        }
    }

    // DFSでp1, p2の位置をキーにすべての取り得る状態を確認する
    let DIR: Vec<i64> = vec![-(n as i64), 1, n as i64, -1];
    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
    let status = (p1, p2);
    dp.insert(status, 0);
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_front(status);

    let mut ans = -1;
    while !que.is_empty() {
        let (p1, p2) = que.pop_back().unwrap();
        let d = dp[&(p1, p2)];
        if p1 == p2 {
            ans = d as i64;
            break;
        }
        for di in DIR.iter() {
            let mut next_p1 = p1 as i64 + di;
            let mut next_p2 = p2 as i64 + di;
            if  (di == &1 && (next_p1%(n as i64))==0) || (di == &-1 && (next_p1%(n as i64))==(n as i64)-1) || next_p1 < 0 || next_p1 >= (n*n) as i64 {
                next_p1 = p1 as i64;
            } else if map[next_p1 as usize] == "#" {
                next_p1 = p1 as i64;
            }
            if  (di == &1 && (next_p2%(n as i64))==0) || (di == &-1 && (next_p2%(n as i64))==(n as i64)-1) || next_p2 < 0 || next_p2 >= (n*n) as i64 {
                next_p2 = p2 as i64;
            } else if map[next_p2 as usize] == "#" {
                next_p2 = p2 as i64;
            }
            let status = (next_p1 as usize, next_p2 as usize);
            if !dp.contains_key(&status) {
                dp.insert(status, d+1);
                que.push_front(status);
            }
        }
    }

    // println!("{:?}", dp);
    println!("{}", ans);
}