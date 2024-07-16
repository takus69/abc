use proconio::input;

fn main() {
    input! {
        a: [[usize; 9]; 9],
    }

    fn check_row(i: usize, a: &[Vec<usize>]) -> bool {
        let mut tmp = Vec::new();
        for j in 0..9 {
            tmp.push(a[i][j]);
        }
        tmp.sort();

        tmp == [1,2,3,4,5,6,7,8,9]
    }

    fn check_col(j: usize, a: &[Vec<usize>]) -> bool {
        let mut tmp = Vec::new();
        for i in 0..9 {
            tmp.push(a[i][j]);
        }
        tmp.sort();

        tmp == [1,2,3,4,5,6,7,8,9]
    }

    fn check_block(k: usize, a: &[Vec<usize>]) -> bool {
        let mut tmp = Vec::new();
        let k1 = (k/3)*3;
        let k2 = (k%3)*3;
        for i in k1..(k1+3) {
            for j in k2..(k2+3) {
                tmp.push(a[i][j]);
            }
        }
        tmp.sort();

        tmp == [1,2,3,4,5,6,7,8,9]
    }

    let mut ans = "Yes";
    for i in 0..9 {
        if !(check_row(i, &a) && check_col(i, &a) && check_block(i, &a)) {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}