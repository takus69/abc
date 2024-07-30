use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut c: Vec<usize> = Vec::new();
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            ci: usize,
            ai: [usize; ci],
        }
        c.push(ci);
        a.push(ai);
    }
    input! {
        x: usize,
    }
    let mut ans: Vec<usize> = Vec::new();
    let mut cnt: usize = usize::MAX;
    for i in 0..n {
        if a[i].contains(&x) {
            if cnt > c[i] {
                cnt = c[i];
                ans = vec![i+1];
            } else if cnt == c[i] {
                ans.push(i+1);
            }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}