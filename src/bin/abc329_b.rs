use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut no1: usize = 0;
    let mut no2: usize = 0;
    for ai in a.iter() {
        no1 = no1.max(*ai);
    }
    for ai in a.iter() {
        if no1 != *ai {
            no2 = no2.max(*ai);
        }
    }
    println!("{}", no2);
}