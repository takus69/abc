use proconio::input;

fn main() {
    input! {
        a: [usize; 5],
    }
    let mut ans: Vec<(usize, String)> = vec![];
    let s = vec!["A", "B", "C", "D", "E"];
    for i in 1..32 {
        let mut name = "".to_string();
        let mut score = 0;
        for j in 0..5 {
            if (i>>j) & 1 == 1 {
                name += s[j];
                score += a[j];
            }
        }
        ans.push((score, name));
    }
    ans.sort_by(|a, b| {b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1))});
    for (score, name) in ans.iter() {
        println!("{}", name);
    }
}