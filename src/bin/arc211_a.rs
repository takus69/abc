use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: [usize; 9],
        }

        let sum_a: usize = a.iter().sum();
        let ans = if a[4] > sum_a-a[4]+1 {
            a[4]*2 - (sum_a+1)
        } else {
            let mut ret = 0;
            for i in 0..4 {
                if a[i] > 0 && a[8-i] > 0 && sum_a == (a[i]+a[8-i]) {
                    ret = 1;
                    break;
                }
            }
            ret
        };
        println!("{}", ans);
    }
}