use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut t: Vec<usize> = Vec::new();
    let mut k: Vec<usize> = Vec::new();
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            ti: usize,
            ki: usize,
            ai: [usize; ki],
        }
        t.push(ti);
        k.push(ki);
        a.push(ai);
    }
    let mut get: Vec<bool> = vec![false; n];
    println!("{}", dfs(n-1, &a, &t, &mut get));

    fn dfs(n: usize, a: &Vec<Vec<usize>>, t: &Vec<usize>, get: &mut Vec<bool>) -> usize {
        if get[n] { return 0; }
        let mut ret = t[n];
        for &ai in a[n].iter() {
            ret += dfs(ai-1, a, t, get);
            get[ai-1] = true;
        }

        ret
    }
}