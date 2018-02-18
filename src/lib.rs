#![crate_name = "ruby_primes_with_rust"]

extern crate primal;

#[no_mangle]
pub fn nth_prime(n: i64) -> i64 {
    if n <= 0 { return -1; }
    let result = primal::StreamingSieve::nth_prime(n as usize);
    result as i64
}

#[cfg(test)]
mod tests {
    use super::nth_prime;

    #[test]
    fn zero_prime_index_is_invalid() {
        assert_eq!(nth_prime(0), -1);
    }

    #[test]
    fn negative_prime_index_is_invalid() {
        assert_eq!(nth_prime(-1), -1);
    }

    #[test]
    fn first_prime_index_is_valid() {
        assert_eq!(nth_prime(1), 2);
    }
}
