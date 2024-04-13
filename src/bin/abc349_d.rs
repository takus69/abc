use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
    }
    let mut ans = Vec::new();
    let mut l = l;
    if l == 0 {
        let mut j = 0;
        while 1<<j <= r {
            j += 1;
        }
        ans.push((l, 1<<(j-1)));
        l = 1<<(j-1);
    }
    while l < r {
        let mut i = 0;
        // println!("l: {}, r: {}", l, r);
        while l%2 == 0 {
            i += 1;
            l /= 2;
        } 
        let mut j = l;
        // println!("i: {}, j: {}", i, j);
        while (1<<i) * (j+1) > r {
            i -= 1;
            j *= 2;
        } 
        // println!("lm: {}, rm: {}", (1<<i)*j, (1<<i)*(j+1));
        ans.push(((1<<i)*j, (1<<i)*(j+1)));
        l = (1<<i)*(j+1)
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{} {}", a.0, a.1);
    }

}