use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        }
        a.sort();
        a.reverse();
        b.sort();
        let mut c = 0;
        let mut i = 0;
        let mut j = 0;
        while j < n {
            if a[i]+b[j] >= m {
                c += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }

        }
        println!("{}", a.iter().sum::<usize>()+b.iter().sum::<usize>()-c*m);
    }
}