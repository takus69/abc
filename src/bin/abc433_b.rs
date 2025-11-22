use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    for (i, &ai) in a.iter().enumerate() {
        let mut max_aj = 0;
        let mut max_j = 0;
        for j in 0..i {
            if ai < a[j] {
                max_aj = a[j];
                max_j = j;
            }
        }
        if ai < max_aj {
            println!("{}", max_j+1);
        } else {
            println!("-1");
        }
    }
}