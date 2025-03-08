use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut vec: Vec<usize> = vec![0; 100];
    for _ in 0..q {
        input! {
            qi: usize,
        }
        if qi == 1 {
            input! {
                x: usize,
            }
            vec.push(x);
        } else {
            let x = vec.pop().unwrap();
            println!("{}", x);
        }
    }
}