use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans: Vec<usize> = vec![n, n*(n-1)/2];
    let mut diff: Vec<(usize, usize)> = Vec::new();
    for i in 0..(n-1) {
        for j in (i+1)..(n) {
            diff.push((j, a[j]-a[i]));
        }
    }
    let mut cnt = 0;
    for i in 0..diff.len() {
        let (jj, d) = diff[i];
        for j in (jj+1)..n {
            if a[j]-a[jj]==d {
                cnt += 1;
            }
        }
    }


    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}