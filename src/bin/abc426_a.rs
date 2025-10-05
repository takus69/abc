use proconio::input;

fn main() {
    input! {
        x: String,
        y: String,
    }
    if &y == "Ocelot" {
        println!("Yes");
    } else if &y == "Serval" && &x != "Ocelot" {
        println!("Yes");
    } else if &y == "Lynx" && &x == "Lynx" {
        println!("Yes");
    } else {
        println!("No");
    }
}