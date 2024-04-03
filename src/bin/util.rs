fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        for j in ((i*2)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

fn main() {
    let msg = "test for primes";
    assert_eq!(primes(6), vec![2, 3, 5], "{}", msg);
    assert_eq!(primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19], "{}", msg);
}