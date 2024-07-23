use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        c: [usize; 9],
    }
    let mut ok = 0;
    let mut ng = 0;
    for perm in (0..9).permutations(9) {
        let mut board = [0; 9];
        let mut ok_flg = true;
        for p in perm.iter() {
            board[*p] = c[*p];
            // 行のチェック
            for i in 0..3 {
                for j in 0..3 {
                    if board[i*3+j] == 0 && board[i*3+(j+1)%3] != 0 && board[i*3+(j+1)%3] == board[i*3+(j+2)%3] {
                        ok_flg = false;
                        break;
                    }
                }
                if !ok_flg { break; }
            }
            // 列のチェック
            for j in 0..3 {
                for i in 0..3 {
                    if board[i*3+j] == 0 && board[((i+1)%3)*3+j] != 0 && board[((i+1)%3)*3+j] == board[((i+2)%3)*3+j] {
                        ok_flg = false;
                        break;
                    }
                }
                if !ok_flg { break; }
            }
            // 斜めのチェック
            for i in 0..3 {
                if board[i*3+i] == 0 && board[((i+1)%3)*3+(i+1)%3] != 0 && board[((i+1)%3)*3+(i+1)%3] == board[((i+2)%3)*3+(i+2)%3] {
                    ok_flg = false;
                    break;
                }
                if board[i*3+(2-i)] == 0 && board[((i+1)%3)*3+(4-i)%3] != 0 && board[((i+1)%3)*3+(4-i)%3] == board[((i+2)%3)*3+(3-i)%3] {
                    ok_flg = false;
                    break;
                }
            }
            if !ok_flg { break; }
        }
        if ok_flg {
            ok += 1;
        } else {
            ng += 1;
        }
    }
    // println!("ok: {}, ng: {}", ok, ng);
    println!("{}", ok as f64 /(ok as f64 + ng as f64));
}