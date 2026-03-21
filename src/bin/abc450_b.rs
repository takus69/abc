use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut cost: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..(n-1) {
        input! {
            tmp_c: [usize; n-1-i],
        }
        cost[i].push(0);
        for (j, &cj) in tmp_c.iter().enumerate() {
            cost[i].push(cj);
            cost[i+j+1].push(cj);
        }
    }
    for a in 0..n {
        for b in (a+1)..n {
            for c in (b+1)..n {
                if a == b || b == c || c == a { continue; }
                if cost[a][b] + cost[b][c] < cost[a][c] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}