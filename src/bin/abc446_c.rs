use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        }

        let mut que: VecDeque<usize> = VecDeque::new();
        let mut last_d = 0;
        for i in 0..n {
            if que.is_empty() { last_d = i+1; }
            que.push_front(a[i]);
            let mut bi = b[i];
            while bi > 0 {
                // 朝
                let c = que.pop_back().unwrap();
                // 昼
                if c <= bi {
                    bi -= c;
                    last_d += 1;
                } else {
                    que.push_back(c-bi);
                    bi = 0;
                }
            }
            // println!("que: {:?}", que);
            // 夜
            // println!("i+1: {}, last_d: {}, d: {}", i+1, last_d, d);
            if i+1>=last_d && i+1 - last_d >= d {
                for _ in 0..=(i+1-last_d-d) {
                    que.pop_back();
                    last_d += 1;
                }
            }
            // println!("que: {:?}", que);
        }
        println!("{}", que.into_iter().sum::<usize>());
    }
}