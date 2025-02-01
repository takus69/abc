use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: Chars,
    }
    let mut b = a.clone();
    let mut back_b: Vec<Vec<char>> = Vec::new();
    while b.len() > 1 {
        back_b.push(b.clone());
        let mut tmp = Vec::new();
        for i in 0..(b.len()/3) {
            let mut b2 = b[(3*i)..(3*(i+1))].to_vec();
            b2.sort();
            if b2[1] == '1' {
                tmp.push('1');
            } else {
                tmp.push('0');
            }
        }
        b = tmp;
    }
    // println!("{}, {}", b[0], b.len());
    let check = if b[0] == '1' { '0' } else { '1' };
    back_b.reverse();
    fn change_cnt(i: usize, j: usize, check: char, back_b: &Vec<Vec<char>>) -> usize {
        let n = back_b.len();
        let mut b = back_b[i][j..(j+3)].to_vec();
        
        let mut ans = 0;
        if i == n-1 {
            if b[0] != check && b[1] != check && b[2] != check {
                return 2;
            } else {
                return 1;
            }
        } else {
            let ans1 = change_cnt(i+1, 3*j, check, &back_b);
            let ans2 = change_cnt(i+1, 3*j+3, check, &back_b);
            let ans3 = change_cnt(i+1, 3*j+6, check, &back_b);
            if b[0] != check && b[1] != check && b[2] != check {
                ans = ans1+ans2+ans3 - ans1.max(ans2).max(ans3);
            } else if b[0] == check {
                ans = ans2.min(ans3);
            } else if b[1] == check {
                ans = ans1.min(ans3);
            } else {
                ans = ans1.min(ans2);
            }
        }
        ans
    }
    println!("{}", change_cnt(0, 0, check, &back_b));
}