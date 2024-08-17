use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }
    let mut all: Vec<Vec<usize>> = Vec::new();
    let mut tmp = vec![1; n];
    all.push(tmp.clone());
    while tmp != r {
        for i in (0..n).rev() {
            tmp[i] += 1;
            if tmp[i] > r[i] {
                tmp[i] = 1;
            } else {
                break;
            }
        }
        all.push(tmp.clone());
    }
    let mut ans: Vec<Vec<usize>> = Vec::new();
    for a in all.iter() {
        if a.iter().sum::<usize>() % k == 0 {
            ans.push(a.clone());
        }
    }
    for a in ans.iter() {
        println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}