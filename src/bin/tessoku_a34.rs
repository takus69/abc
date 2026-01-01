use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }
    let mut groudy: Vec<usize> = Vec::new();
    let max_a: usize = *(a.iter().max().unwrap());
    for i in 0..=max_a{
        if i < x {
            groudy.push(0);
        } else {
            let mut gs: Vec<usize> = vec![groudy[i-x]];
            if i >= y {
                gs.push(groudy[i-y]);
            }
            for gi in [0, 1, 2] {
                if !gs.contains(&gi) {
                    groudy.push(gi);
                    break;
                }
            }
        }
    }
    let mut ans = 0;
    for &ai in a.iter() {
        ans ^= groudy[ai];
    }
    if ans == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}