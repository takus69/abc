use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut mass: Vec<usize> = vec![0, 0, 0, 0];
    let mut ans = 0;
    for &ai in a.iter() {
        mass[0] += 1;
        if ai > 0 {
            for j in (0..4).rev() {
                if ai + j < 4 {
                    mass[ai+j] = mass[j];
                } else {
                    ans += mass[j];
                }
                mass[j] = 0;
            }
        }
    }
    println!("{}", ans);
}