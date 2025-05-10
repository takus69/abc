use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut cnt = 0;
    let mut flg: Vec<bool> = vec![false; m];
    for &ai in a.iter() {
        flg[ai-1] = true;
        cnt += 1;
        let mut all = true;
        for &bi in flg.iter() {
            if !bi {
                all = false;
                break;
            }
        }
        if all {
            println!("{}", n-cnt+1);
            return;
        }
    }
    println!("0");
}