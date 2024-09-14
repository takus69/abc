use proconio::input;

fn main() {
    input! {
        n: i64,
        x: [i64; n],
        p: [i64; n],
        q: i64,
    }
    let mut sum = 0;
    let mut sum_p: Vec<i64> = vec![sum];
    for pi in p.iter() {
        sum += pi;
        sum_p.push(sum);
    }

    for _ in 0..q {
        input! {
            l: i64,
            r: i64,
        }
        let mut li = match x.binary_search(&l) {
            Ok(i) => {i},
            Err(i) => {i},
        };
        let mut ri = match x.binary_search(&(r+1)) {
            Ok(i) => {i},
            Err(i) => {i},
        };
        println!("{}", sum_p[ri] - sum_p[li]);

    }
}