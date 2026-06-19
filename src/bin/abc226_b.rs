use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a: Vec<Vec<usize>> = Vec::new();
    let mut l: Vec<usize> = Vec::new();
    for _ in 0..n {
        input! {
            li: usize,
            ai: [usize; li],
        }
        l.push(li);
        a.push(ai);
    }
    a.sort_by(|x, y| (x.len(), x).cmp(&(y.len(), y)));
    let mut pre_l = 0;
    let mut pre_a: Vec<usize> = Vec::new();
    let mut ans = 0;
    for ai in a {
        if pre_l != ai.len() || pre_a != ai {
            ans += 1;
        }
        pre_l = ai.len();
        pre_a = ai;
    }
    println!("{}", ans);
}