use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let n3 = ((n as f64).powf(1.0/3.0) as usize) + 1;
    let mut nn = Vec::new();
    for i in 1..=n3 {
        if n % i == 0 {
            nn.push(i);
        }
    }

    for &k in nn.iter() {
        if n/k <= k*k { continue; }
        let c = n/k - k*k;
        if c%3 != 0 { continue; }
        let c = c/3;
        let y = k*k + 4*c;
        let y = (y as f64).sqrt() as usize;
        if y <= k { continue; }
        let y = y - k;
        if y % 2 != 0 { continue; }
        let y = y / 2;
        let x = y + k;
        if x*x + x*y + y*y == n/k {
            println!("{} {}", x, y);
            std::process::exit(0);
        }
    }

    println!("-1");
}