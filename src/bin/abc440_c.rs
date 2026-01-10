use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            w: usize,
            c: [usize; n],
        }
        let mut sum_2w: Vec<usize> = vec![0; w*2];
        for i in 0..n {
            let idx = i%(2*w);
            sum_2w[idx] += c[i];
        }
        // println!("sum_2w: {:?}", sum_2w);
        sum_2w.extend(sum_2w.clone());
        // println!("sum_2w: {:?}", sum_2w);
        let mut s = 0;
        for i in 0..w {
            s += sum_2w[i];
        }
        let mut x = 1;
        let mut min_s = s;
        for i in 1..(2*w) {
            // println!("i: {}, s: {}", i, s);
            s -= sum_2w[i-1];
            s += sum_2w[w+i-1];
            if s < min_s {
                min_s = s;
                x = i+1;
            }
        }
        println!("{}", min_s);
    }
}