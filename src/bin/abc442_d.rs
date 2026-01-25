use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    }
    let mut fw: FenwickTree<isize> = FenwickTree::new(n, 0);
    for (i, &ai) in a.iter().enumerate() {
        fw.add(i, ai);
    }
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                x: usize,
            }
            let x1 = fw.sum((x-1)..x);
            let x2 = fw.sum(x..=x);
            fw.add(x-1, -x1);
            fw.add(x, -x2);
            fw.add(x-1, x2);
            fw.add(x, x1);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", fw.sum((l-1)..r));
        }
    }
}