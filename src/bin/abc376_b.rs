use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, usize); q],
    }
    let mut ans = 0;
    let mut lr = [1, 2];
    for i in 0..q {
        let (h, t) = ht[i];
        let j = if h == 'L' {0} else {1};
        let mut tmp_ans;
        let mut tmp = 0;
        for k in 1..n {
            let kk = (lr[j] + k - 1)%n+1;
            if (lr[j] + k - 1)%n+1 == lr[(j+1)%2] {
                tmp = usize::MAX;
                break;
            } else if (lr[j] + k - 1)%n+1 == t {
                tmp += 1;
                break;
            } else {
                tmp += 1;
            }
        }
        tmp_ans = tmp;
        let mut tmp = 0;
        for k in (1..n).rev() {
            let kk = (lr[j] + k - 1)%n+1;
            if (lr[j] + k - 1)%n+1 == lr[(j+1)%2] {
                tmp = usize::MAX;
                break;
            } else if (lr[j] + k - 1)%n+1 == t {
                tmp += 1;
                break;
            } else {
                tmp += 1;
            }
        }
        tmp_ans = tmp_ans.min(tmp);
        lr[j] = t;
        ans += if tmp_ans != usize::MAX { tmp_ans } else { 0 };
    }
    println!("{}", ans);

}
