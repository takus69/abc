use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    
    let mut que: VecDeque<String> = VecDeque::new();
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    x: String,
                }
                que.push_back(x);
            },
            2 => {
                let x = que.front().unwrap();
                println!("{}", x);
            },
            3 => {
                que.pop_front();
            },
            _ => {},
        }
    }
}