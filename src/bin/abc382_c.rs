use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut min_a: Vec<usize> = Vec::new();
    let mut tmp= usize::MAX;
    for &ai in a.iter() {
        tmp = tmp.min(ai);
        min_a.push(tmp);
    }
    min_a.reverse();
    for &bi in b.iter() {
        let pos = min_a.partition_point(|&x| x <= bi);
        if bi < min_a[0] {
            println!("-1");
        } else {
            println!("{}", n-pos+1);
        }
    }
}