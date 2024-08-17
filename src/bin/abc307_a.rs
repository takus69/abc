use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7*n],
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        ans.push(a[i*7..(i+1)*7].iter().sum());
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}