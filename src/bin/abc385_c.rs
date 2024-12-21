use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut list: Vec<Vec<usize>> = vec![vec![]; 3001];
    for (i, &hi) in h.iter().enumerate() {
        list[hi].push(i);
    }
    let mut ans = 0;
    for v in list.iter() {
        for (i, &vi) in v.iter().enumerate() {
            for j in 1..3000 {
                let mut tmp = 1;
                let hi = h[vi];
                let mut vi = vi;
                while vi + j < h.len() {
                    if h[vi + j] == hi {
                        vi += j;
                        tmp += 1;
                    } else {
                        break;
                    }
                }
                ans = ans.max(tmp);
            }
        }
    }
    println!("{}", ans);
}