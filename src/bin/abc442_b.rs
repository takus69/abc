use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut player = false;
    let mut volume = 0;
    for _ in 0..q {
        input! {
            a: usize,
        }
        match a {
            1 => { volume += 1; },
            2 => { if volume > 0 { volume -= 1; }},
            3 => { player = !player; }
            _ => {},
        }
        if player && volume >= 3 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}