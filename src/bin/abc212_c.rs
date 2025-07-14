use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();

    let mut ans = a[n-1].max(b[m-1]) - a[0].min(b[0]);
    for &ai in a.iter() {
        let j = b.partition_point(|&x| x < ai);
        if j < m {
            ans = ans.min(ai.abs_diff(b[j]));
        }
        if j > 0 {
            ans = ans.min(ai.abs_diff(b[j-1]));
        }

        let j = b.partition_point(|&x| x <= ai);
        if j < m {
            ans = ans.min(ai.abs_diff(b[j]));
        }
    }
    println!("{}", ans);
}