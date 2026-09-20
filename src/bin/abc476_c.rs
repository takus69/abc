use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut top3: Vec<usize> = vec![a[0], a[1], a[2]];
    top3.sort();top3.reverse();
    println!("{}", top3[2]);

    for &ai in a.iter().skip(3) {
        top3.push(ai);
        top3.sort();top3.reverse();
        top3.pop();
        println!("{}", top3[2]);
    }
}
