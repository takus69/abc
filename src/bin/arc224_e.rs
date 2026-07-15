use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut s: Chars,
        }
        let mut stack: Vec<char> = Vec::new();
        while let Some(si) = s.pop() {
            stack.push(si);
            let n = stack.len();
            if n > 2 {
                if stack[n-1]=='A' && stack[n-2]=='B' && stack[n-3]=='C' {
                    stack.pop(); stack.pop(); stack.pop();
                }
            }
            let n = stack.len();
            if n > 1 {
                if stack[n-1]=='A' && stack[n-2]=='B' {
                    stack.pop(); stack.pop();
                }
            }
            let n = stack.len();
            if n > 0 {
                if stack[n-1]=='A' {
                    stack.pop();
                }
            }
        }
        println!("{}", stack.len());
    }
}