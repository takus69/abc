use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            mut ai: [usize; k],
        }
        ai.reverse();
        a.push(ai);
    }
    const EMPTY: usize = usize::MAX;
    let mut have: Vec<usize> = vec![EMPTY; n+1];
    let mut stack: Vec<usize> = (0..m).into_iter().collect();
    let mut cnt = 0;
    while let Some(i) = stack.pop() {
        if let Some(top) = a[i].pop() {
            if have[top] == EMPTY {
                have[top] = i;
            } else {
                stack.push(have[top]);
                stack.push(i);
                have[top] = EMPTY;
                cnt += 2;
            }
        }
    }
    if cnt == 2*n {
        println!("Yes");
    } else {
        println!("No");
    }
}