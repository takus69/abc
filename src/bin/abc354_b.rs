use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(String, usize); n],
    }

    let mut sc = sc;
    sc.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum_c = 0;
    for (_, c) in sc.iter() {
        sum_c += c;
    }
    let ans_i = sum_c % n;

    println!("{}", sc[ans_i].0);
}