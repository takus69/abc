use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }

    let mut row_bingo = vec![0; n];
    let mut col_bingo = vec![0; n];
    let mut cross_bingo = vec![0; 2];
    let mut ans: i64 = -1;

    for (i, ai) in a.iter().enumerate() {
        let r = (ai-1) / n;
        let c = (ai-1) % n;
        row_bingo[r] += 1;
        if row_bingo[r] == n {
            ans = i as i64 +1;
            break;
        }
        col_bingo[c] += 1;
        if col_bingo[c] == n {
            ans = i as i64 +1;
            break;
        }
        if r == c {
            cross_bingo[0] += 1;
            if cross_bingo[0] == n {
                ans = i as i64 +1;
                break;
            }
        }
        if r+c == n-1 {
            cross_bingo[1] += 1;
            if cross_bingo[1] == n {
                ans = i as i64 +1;
                break;
            }
        }
    }

    println!("{}", ans);
}