use proconio::input;
fn main(){
input!{n:usize,t:u32,a:[usize;t]}
let mut b=vec![0;n*2+2];
for(i,ai)in a.iter().enumerate(){
    let r=(ai-1)/n;
    let c=(ai-1)%n+n;
    b[r]+=1;
    b[c]+=1;
    if r==c-n{b[2*n]+=1};
    if r+c-n==n-1{b[2*n+1]+=1;}
    if b[r]==n||b[c]==n||b[2*n]==n||b[2*n+1]==n{
        println!("{}",i+1);
        return;
    }
}
println!("-1");
}