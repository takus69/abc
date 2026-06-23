use proconio::input;

fn find(x: usize, parent: &mut [usize]) -> usize {
    if parent[x] == x {
        x
    } else {
        let p = find(parent[x], parent);
        parent[x] = p;
        p
    }
}

fn main() {
    input! {
        q: usize,
    }
    let n = 2usize.pow(20);
    let mut parent: Vec<usize> = (0..n).collect();
    let mut ans: Vec<isize> = vec![-1; n];
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 1 {
            let p = find(x%n, &mut parent);
            ans[p] = x as isize;
            parent[p] = (p+1)%n;
        } else {
            println!("{}", ans[x%n]);
        }
    }
}