use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut x: [usize; n],
        a: [usize; n],
    }
    for i in 0..n {
        x[i] -= 1;
    }
    let mut p: Vec<Vec<usize>> = vec![x.clone()];  // p[i][j]: 2のi乗の時のjの要素番号
    for i in 1..=60 {
        let mut tmp: Vec<usize> = Vec::new();
        for j in 0..n {
            tmp.push(p[i-1][p[i-1][j]]);
        }
        p.push(tmp);
    }
    let mut x: Vec<usize> = (0..n).collect();
    // println!("x: {:?}", x);
    for i in 0..=60 {
        if k % 2 == 1 {
            for j in 0..n {
                x[j] = p[i][x[j]];
            }       
        }
        // println!("x: {:?}", x);
        k /= 2;
        if k == 0 { break; }
    }
    // println!("x: {:?}", x);
    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(a[x[i]]);
    }
    // println!("p: {:?}", p);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}