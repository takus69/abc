use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v: Vec<usize> = Vec::new();
    let mut tmp = 0;
    for &ai in a.iter() {
        v.push(ai);
        if v.len() >= 4 {
            let i = v.len()-1;
            if v[i]==v[i-1] && v[i]==v[i-2] && v[i]==v[i-3] {
                v.pop();v.pop();v.pop();v.pop();
            }
        }
    }

    println!("{}", v.len());
}