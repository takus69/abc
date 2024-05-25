use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let mut ans = Vec::new();
    let mut min_c: usize = 1_000_000_000;

    let mut iac = Vec::new();
    for (i, (a, c)) in ac.into_iter().enumerate() {
        iac.push((i, a, c));
    }
    iac.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, a, c) in iac.iter() {
        if min_c >= *c {
            ans.push(i+1);
            min_c = *c;
        }
    }
    ans.sort();

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}