use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    fn f(ai: usize, m: usize, n: usize, cnt: usize) -> (bool, Vec<Vec<usize>>) {
        let mut v = Vec::new();

        let cnt = cnt + 1;
        // println!("ai: {}, cnt: {}", ai, cnt);
        if ai+(n-cnt)*10 > m { return (false, v); }
        if cnt == n {
            v.push(vec![ai]);
            return (true, v);
        }
        
        for i in (ai+10)..=m {
            let (b, vec) = f(i, m, n, cnt);
            if !b { break; }
            for mut v2 in vec.into_iter() {
                v2.push(ai);
                v.push(v2);
            }
        }

        // println!("v: {:?}", v);
        (true, v)
    }

    let mut ans = Vec::new();
    for ai in 1..(m+1) {
        let (b, vec) = f(ai, m, n, 0);
        if !b { break; }
        for mut v in vec.into_iter() {
            if v.len() != n { continue; }
            v.reverse();
            ans.push(v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
    println!("{}", ans.len());
    for a in ans.iter() {
        println!("{}", a);
    }
}