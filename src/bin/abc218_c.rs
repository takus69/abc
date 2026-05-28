use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    }

    fn trim(s: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut min_i = usize::MAX;
        let mut min_j = usize::MAX;
        let mut max_i = 0;
        let mut max_j = 0;
        for i in 0..s.len() {
            for j in 0..s[0].len() {
                if s[i][j] == '#' {
                    min_i = min_i.min(i);
                    min_j = min_j.min(j);
                    max_i = max_i.max(i);
                    max_j = max_j.max(j);
                }
            }
        }
        let mut ret: Vec<Vec<char>> = Vec::new();
        for i in min_i..=max_i {
            let mut tmp: Vec<char> = Vec::new();
            for j in min_j..=max_j {
                tmp.push(s[i][j]);
            }
            ret.push(tmp);
        }

        ret
    }

    fn rotate(s: &[Vec<char>]) -> Vec<Vec<char>> {
        let r = s.len();
        let c = s[0].len();
        let mut ret: Vec<Vec<char>> = vec![vec!['.'; r]; c];
        for i in 0..r {
            for j in 0..c {
                ret[j][r-i-1] = s[i][j];
            }
        }

        ret
    }

    let ti = trim(&t);
    // println!("t: {:?}", ti);
    for _ in 0..4 {
        s = rotate(&s);
        let si = trim(&s);
        // println!("s: {:?}", si);
        if ti == si {
            println!("Yes");
            return;
        }
    }
    println!("No");
}