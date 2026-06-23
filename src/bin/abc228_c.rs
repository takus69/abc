use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    }
    let mut rank: Vec<usize> = Vec::new();
    for i in 0..n {
        rank.push(p[i].iter().sum());
    }
    rank.sort();
    let mut score = 0;
    for _ in 0..k {
        let tmp = rank.pop().unwrap();
        score = tmp;
    }
    for i in 0..n {
        if p[i].iter().sum::<usize>()+300 >= score {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}