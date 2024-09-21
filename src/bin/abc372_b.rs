use proconio::input;

fn main() {
    input! {
        mut m: usize,
    }
    let mut ans: Vec<u32> = vec![];
    let mut i = 10;
    while m > 0 {
        let d = 3_usize.pow(i);
        if m < d {
            i -= 1;
            continue;
        }
        m -= d;
        ans.push(i);
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}