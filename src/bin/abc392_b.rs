use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 1..=n {
        if !a.contains(&i) {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}