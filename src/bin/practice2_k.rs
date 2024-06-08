use proconio::input;
use ac_library::{Segtree, Monoid, ModInt998244353};

type Mint = ModInt998244353;
struct Sum;
impl Monoid for Sum {
    type S = Mint;

    fn identity() -> Mint {
        Mint::new(0)
    }

    fn binary_operation(a: &Mint, b: &Mint) -> Mint {
        a + b
    }

}
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut sg = Segtree::<Sum>::new(n);
    for (i, ai) in a.iter().enumerate() {
        sg.set(i, Mint::new(*ai));
    }
    
    for _ in 0..q {
        input! { x: usize }
        if x == 0 {
            input! {
                l: usize,
                r: usize,
                b: usize,
                c: usize,
            }
            for i in l..r {
                let ai = sg.get(i);
                sg.set(i, Mint::new(b) * ai + Mint::new(c));
            }
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let ans = sg.prod(l..r);
            println!("{}", ans);
        }
    }
}