use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut stack: Vec<String> = Vec::new();
    for _ in 0..q {
        input! {
            c: usize,
        }

        match c {
            1 => {
                input! {
                    x : String,
                }
                stack.push(x);
            },
            2 => {
                let x = stack.last().unwrap();
                println!("{}", x);
            },
            3 => {
                stack.pop();
            },
            _ => {},
        }
    }
}