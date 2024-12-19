use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a: Vec<Vec<usize>> = Vec::new();
    for i in 1..=n {
        input! {
            ai: [usize; i],
        }
        a.push(ai);
    }
    let mut ans = 1;
    for j in 1..=n {
        let i = ans; 
        let (i, j) = if ans < j {
            (j-1, ans-1)
        } else {
            (ans-1, j-1)
        };
        ans = a[i][j];
    }
    println!("{}", ans);
}