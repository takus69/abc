use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
        mut b: [usize; m],
    }
    for i in 0..m {
        a[i] -= 1;
        b[i] -= 1;
    }
    let mut array = vec![-1; n];
    let mut ans = "Yes";
    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut que: VecDeque<(usize, i32)> = VecDeque::new();
    for i in 0..n {
        link.insert(i, Vec::new());
    }
    for i in 0..m {
        link.get_mut(&a[i]).unwrap().push(b[i]);
        link.get_mut(&b[i]).unwrap().push(a[i]);
    }
    for i in 0..n {
        if array[i] != -1 { continue; }
        que.push_front((i, 0));
        while !que.is_empty() {
            let (j, x) = que.pop_back().unwrap();
            if array[j] == x {
                continue;
            } else if array[j] == -1 {
                array[j] = x;
            } else {
                ans = "No";
                break;
            }
            let next_x = (x+1)%2;
            for k in link[&j].iter() {
                if array[*k] == next_x {
                    continue;
                } else if array[*k] == -1 {
                    que.push_front((*k, next_x));
                } else {
                    ans = "No";
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}