use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    }
    let si = (b[0][0]-1)/7+1;
    let sj = (b[0][0]-1)%7+1;
    let ti = (b[n-1][m-1]-1)/7+1;
    let tj = (b[n-1][m-1]-1)%7+1;
    // println!("si: {}, sj: {}, ti: {}, tj: {}", si, sj, ti, tj);
    if si > ti || sj > tj || n < ti-si+1 || m < tj-sj+1 {
        println!("No");
        return;
    }
    for i in si..=ti {
        for j in sj..=tj {
            // println!("b: {}, (i-1)*7+j: {}", b[i-si][j-sj], (i-1)*7+j);
            if b[i-si][j-sj] == (i-1)*7+j { continue; }
            println!("No");
            return;
        }
    }
    println!("Yes");
}