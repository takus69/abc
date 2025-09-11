use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut best_hand: Vec<usize> = vec![0; n+1];  // 奇数先手勝ち、偶数後手勝ち

    fn f(i: usize, a: usize, b: usize, best_hand: &mut Vec<usize>) -> usize {
        if best_hand[i] != 0 { return best_hand[i]; }
        if i < a {
            best_hand[i] = 2;
        } else if i >= a && i < b+a {
            best_hand[i] = 1;
        } else {
            if f(i-a, a, b, best_hand) == 2 || f(i-b, a, b, best_hand) == 2 {
                best_hand[i] = 1;
            } else {
                best_hand[i] = 2;
            }
        }

        best_hand[i]
    }
    
    if f(n, a, b, &mut best_hand) == 1 {
        println!("First");
    } else {
        println!("Second");
    }
    // println!("{:?}", best_hand);
}