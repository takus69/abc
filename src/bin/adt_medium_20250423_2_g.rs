use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: usize,
        mut a: [usize; n],
    }
    let sum_a: usize = a.iter().sum();
    s %= sum_a;
    let mut i = 0;
    let mut j = 0;
    let mut tmp = 0;
    a.extend(a.clone());
    while j < n*2 {
        if s == tmp {
            println!("Yes");
            return;
        } else if s < tmp {
            i += 1;
            tmp -= a[i];
        } else {
            j += 1;
            if j < n*2 {
                tmp += a[j];
            }
        }
    }
    println!("No");
}