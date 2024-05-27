use proconio::input;
fn main() {
input!{n:i64,m:i64,a:[i64;n],b:[i64;m]}
let mut c=b;
c.extend(&a);
c.sort();
for w in c.windows(2){
if a.contains(&w[0]) && a.contains(&w[1]) {
println!("Yes");
return;}}
println!("No");}