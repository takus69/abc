use proconio::input;

fn main() {
    let a = true;
    let b = false;
    let c = true;

    if (a) {
        print!("At");
    } else {
        print!("Yo");
    }

    if (!a && b) {
        print!("Bo");
    } else if (!b && c) {
        print!("Co");
    }

    if (a && b && c) {
        println!("foo!");
    } else if (true && false) {
        println!("yeah!");
    } else if (!a || c) {
        println!("der");
    }
}