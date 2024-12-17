use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    fn rotate(i: usize, x: usize, y: usize, ans: &mut Vec<Vec<char>>, s: &Vec<Vec<char>>) {
        // sのiの和を回転させて、配置
        let n = s.len();
        match i%4 {
            0 => {
                ans[y][n-x-1] = s[x][y];
            },
            1 => {
                ans[n-x-1][n-y-1] = s[x][y];
            },
            2 => {
                ans[n-y-1][x] = s[x][y];
            },
            3 => {
                ans[x][y] = s[x][y];
            },
            _ => {},
        }
    }
    let mut ans = s.clone();
    for i in 0..(n/2) {
        for x in i..(n-i) {
            let y = i;
            rotate(i, x, y, &mut ans, &s);
            let y = n-i-1;
            rotate(i, x, y, &mut ans, &s);
        }
        for y in i..(n-i) {
            let x = i;
            rotate(i, x, y, &mut ans, &s);
            let x = n-i-1;
            rotate(i, x, y, &mut ans, &s);
        }
    }
    for ai in ans.iter() {
        println!("{}", ai.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}