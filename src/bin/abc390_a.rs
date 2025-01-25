use proconio::input;

fn main() {
    input! {
        a: [usize; 5],
    }
    let mut a2= a.clone();
    a2.sort();
    for i in 0..4 {
        let mut tmp = vec![];
        for j in 0..5 {
            if j == i+1 { continue; }
            if j == i {
                tmp.push(a[i+1]);
                tmp.push(a[i]);
            } else {
                tmp.push(a[j]);
            }
        }
        if a2 == tmp {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}