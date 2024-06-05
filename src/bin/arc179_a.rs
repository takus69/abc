use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
    }
    if k <= 0 {
        let sum_a: i64 = a.iter().sum();
        if sum_a >= k {
            println!("Yes");
            a.sort_by(|x, y| y.cmp(x));
            println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        } else {
            println!("No");
        }
    } else {
        println!("Yes");
        a.sort();
        println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

    }
}