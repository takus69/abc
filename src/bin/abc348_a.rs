use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = String::from("");

    for i in 0..n {
        if (i+1) % 3 == 0 {
            ans.push_str(&String::from("x"));
        } else {
            ans.push_str(&String::from("o"));
        }
    }

    println!("{}", ans);
}
