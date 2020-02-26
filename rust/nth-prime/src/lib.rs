pub fn nth(n: u32) -> u32 {
    let mut prime_count = 0;
    let mut possible_prime = 2;
    loop {
        if is_prime(possible_prime) {
            if prime_count == n {
                break;
            } else {
                prime_count += 1;
            }
        }
        possible_prime += 1;
    }
    possible_prime
}

fn is_prime(n: u32) -> bool {
    !(2..=(n as f64).sqrt() as u32).any(|i| n % i == 0)
}
