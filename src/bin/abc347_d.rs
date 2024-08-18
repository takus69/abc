use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        mut c: usize,
    }
    let mut cnt_c = 0;
    let mut tmp = 1;
    while c >= tmp {
        if c & tmp > 0 {
            cnt_c += 1;
        }
        tmp <<= 1;
    }
    
    // println!("a: {}, b: {}, cnt_c: {}", a, b, cnt_c);
    if a + b < cnt_c {
        println!("-1");
        std::process::exit(0);
    }
    let mut same_cnt = a+b - cnt_c;
    // println!("same_cnt: {}", same_cnt);
    if same_cnt % 2 == 1 {
        println!("-1");
        std::process::exit(0);
    }
    same_cnt /= 2;
    if a.max(b)-a.min(b) > cnt_c {
        println!("-1");
        std::process::exit(0);
    }
    if a + b - same_cnt > 60 {
        println!("-1");
        std::process::exit(0);
    }

    let mut cnt_a = same_cnt;
    let mut cnt_b = same_cnt;
    let mut a_bit: Vec<usize> = Vec::new();
    let mut b_bit: Vec<usize> = Vec::new();
    for _ in 0..=60 {
        // println!("c: {}, cnt_a: {}, cnt_b: {}", c, cnt_a, cnt_b);
        if c%2 == 1 && cnt_a < a {
            a_bit.push(1);
            b_bit.push(0);
            cnt_a += 1;
        } else if c%2 == 1 && cnt_b < b {
            a_bit.push(0);
            b_bit.push(1);
            cnt_b += 1;
        } else if c%2 == 0 && same_cnt > 0 {
            a_bit.push(1);
            b_bit.push(1);
            same_cnt -= 1;
        } else {
            a_bit.push(0);
            b_bit.push(0);
        }

        c /= 2;
    }
    // println!("cnt_a: {}, cnt_b: {}, same_cnt: {}", cnt_a, cnt_b, same_cnt);
    let mut a_ans: i64 = 0;
    let mut b_ans: i64 = 0;
    let mut tmp = 1;
    for i in 0..a_bit.len() {
        if a_bit[i] == 1 {
            a_ans += tmp;
        }
        if b_bit[i] == 1 {
            b_ans += tmp;
        }
        tmp <<= 1;
    }
    println!("{} {}", a_ans, b_ans);
}