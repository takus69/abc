use proconio::input;

fn make_block(b: Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for bi in b.iter() {
        ret.push(bi.repeat(3));
    }
    let w = ".".repeat(b.len());
    for bi in b.iter() {
        ret.push(bi.clone() + &w + bi);
    }
    for bi in b.iter() {
        ret.push(bi.repeat(3));
    }

    ret
}

fn main() {
    input! {
        n: usize,
    }

    let mut b = vec![String::from("#")];
    for _ in 1..=n {
        b = make_block(b.clone());
    }
    b.iter().for_each(|x| println!("{}", x));
}