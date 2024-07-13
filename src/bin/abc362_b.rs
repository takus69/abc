use proconio::input;

fn main() {
    input! {
        xa: i64,
        ya: i64,
        xb: i64,
        yb: i64,
        xc: i64,
        yc: i64,
    }
    let mut ans = "No";
    if (xb - xa)*(xc - xa) + (yb - ya)*(yc - ya) == 0 {
        ans = "Yes";
    } else if (xc - xb)*(xa - xb) + (yc - yb)*(ya - yb) == 0 {
        ans = "Yes";
    } else if (xa - xc)*(xb - xc) + (ya - yc)*(yb - yc) == 0 {
        ans = "Yes";
    }
    println!("{}", ans);
}