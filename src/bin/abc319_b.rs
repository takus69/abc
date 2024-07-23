use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: Vec<String> = Vec::new();
    for i in 0..=n {
        let mut exists = false;
        for j in 1..=9 {
            if n%j == 0  && i%(n/j) == 0 {
                ans.push(format!("{}", j));
                exists = true;
                break;
            }
        }
        if !exists {
            ans.push("-".to_string());
        }
    }
    println!("{}", ans.join(""));
}