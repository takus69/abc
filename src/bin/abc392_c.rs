use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
        mut q: [usize; n],
    }
    for i in 0..n {
        p[i] -= 1;
        q[i] -= 1;
    }
    let mut iq = vec![0; n];
    for (i, &qi) in q.iter().enumerate() {
        iq[qi] = i;
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        ans.push(q[p[iq[i]]]+1);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}