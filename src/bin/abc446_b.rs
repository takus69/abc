use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    
    let mut selected: Vec<bool> = vec![false; m+1];
    for _ in 0..n {
        input! {
            l: usize,
            x: [usize; l],
        }
        let mut flg = false;
        for &xi in x.iter() {
            if !selected[xi] {
                println!("{}", xi);
                selected[xi] = true;
                flg = true;
                break;
            }
        }
        if !flg {
            println!("0");
        }
    }
}