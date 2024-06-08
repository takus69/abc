use proconio::input;

fn make_block(b: Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for bi in b.iter() {
        ret.push(bi.clone() + bi + bi);
    }
    let mut w = String::from("");
    for _ in 0..b.len() {
        w += ".";
    }
    for bi in b.iter() {
        ret.push(bi.clone() + &w + bi);
    }
    for bi in b.iter() {
        ret.push(bi.clone() + bi + bi);
    }

    ret
}

fn main() {
    input! {
        n: usize,
    }

    let mut b = vec![String::from("#")];
    for i in 1..=n {
        b = make_block(b.clone());
    }
    for bi in b.iter() {
        println!("{}", bi);
    }
}