pub fn factors(n: u64) -> Vec<u64> {
    let mut res = n;
    let mut factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;

    while res > 1 {
        if res % divisor == 0 {
            factors.push(divisor);
            res /= divisor;
        } else {
            divisor += 1;
        }
    }
    factors
}
