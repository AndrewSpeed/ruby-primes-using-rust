#![crate_name = "ruby_primes_with_rust"]

extern crate primal;

#[no_mangle]
pub fn nth_prime(n: isize) -> usize {
    if n <= 0 { return 0; }
    primal::StreamingSieve::nth_prime(n as usize)
}

#[cfg(test)]
mod tests {
    use super::nth_prime;

    #[test]
    fn zero_prime_index_is_invalid() {
        assert_eq!(nth_prime(0), 0);
    }

    #[test]
    fn negative_prime_index_is_invalid() {
        assert_eq!(nth_prime(-1), 0);
    }

    #[test]
    fn first_prime_index_is_valid() {
        assert_eq!(nth_prime(1), 2);
    }
}
