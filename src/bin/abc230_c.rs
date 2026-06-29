use proconio::input;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize,
    }
    let mut ans: Vec<String> = vec!["".to_string(); (q-p+1) as usize];
    for i in p..=q {
        for j in r..=s {
            // (i, j)=(A+k, B+k) => k=i-A, k=j-B => i-A=j-B
            // (i, j)=(A+k, B-k) => k=i-A, k=B-j => i-A=B-j
            let mut tmp = '.';
            if i+b==j+a || i+j==b+a {
                tmp = '#';
            }
            ans[(i-p) as usize].push(tmp);
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}