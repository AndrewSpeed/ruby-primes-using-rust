require 'ffi'

module RustPrimes
  extend FFI::Library

  ffi_lib 'libruby_primes_with_rust.so'

  attach_function(:nth_prime, :nth_prime, [:long_long], :long_long)
end

def nth_prime(n)
  result = RustPrimes.nth_prime(n)

  raise 'Invalid prime' if result <= 0
  result
end
