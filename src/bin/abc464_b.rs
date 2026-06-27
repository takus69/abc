use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut s_i = 0;
    let mut t_i = h-1;
    let mut s_j = 0;
    let mut t_j = w-1;
    for i in 0..h {
        let mut flg = true;
        for j in 0..w {
            if c[i][j] == '#' {
                flg = false;
                break;
            }
        }
        if flg {
            s_i = i+1;
        } else {
            break;
        }
    }

    for i in (1..h).rev() {
        let mut flg = true;
        for j in 0..w {
            if c[i][j] == '#' {
                flg = false;
                break;
            }
        }
        if flg {
            t_i = i-1;
        } else {
            break;
        }
    }

    for j in 0..w {
        let mut flg = true;
        for i in 0..h {
            if c[i][j] == '#' {
                flg = false;
                break;
            }
        }
        if flg {
            s_j = j+1;
        } else {
            break;
        }
    }

    for j in (1..w).rev() {
        let mut flg = true;
        for i in 0..h {
            if c[i][j] == '#' {
                flg = false;
                break;
            }
        }
        if flg {
            t_j = j-1;
        } else {
            break;
        }
    }

    let mut ans: Vec<String> = Vec::new();
    for i in 0..=(t_i-s_i) {
        ans.push(String::new());
        for j in 0..=(t_j-s_j) {
            ans[i].push(c[i+s_i][j+s_j]);
        }
    }
    
    for a in &ans {
        println!("{}", a);
    }
}