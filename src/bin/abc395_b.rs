use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans: Vec<Vec<&str>> = vec![vec!["."; n]; n];
    for i in 0..n {
        let j = n-i-1;
        if i > j { continue; }
        for i2 in i..=j {
            for j2 in i..=j {
                if (i+1)%2 == 1 {
                    ans[i2][j2] = "#";
                } else {
                    ans[i2][j2] = ".";
                }
            }
        }
    }
    for ai in ans.iter() {
        println!("{}", ai.join(""));
    }
}