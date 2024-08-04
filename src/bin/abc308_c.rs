use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut rate: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..n {
        let (a, b) = ab[i];
        rate.push((a, a+b, i+1));
    }
    rate.sort_by(|a, b| {
        if b.0*a.1 == a.0*b.1 {
            a.2.cmp(&b.2)
        } else {
            (b.0*a.1).cmp(&(a.0*b.1))
        }
    });
    println!("{}", rate.iter().map(|x| x.2.to_string()).collect::<Vec<String>>().join(" "));
}