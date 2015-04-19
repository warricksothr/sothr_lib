// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(core)] //Required to compile 3/31/2015. Test again later when libs have stablized.

extern crate num;

pub mod str_util;
pub mod prime_util;
pub mod number_util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_for_vector() {
        let vec = vec![1, 2];
        assert_eq!("1,2", str_util::string_for_vector(vec));
    }

    #[test]
    fn test_example() {
        assert_eq!("test", "test");
    }

    #[test]
    fn test_list_of_primes() {
        let limit = 2_000_000u64;
        let mut primes = Vec::new();
        for i in 2..limit {
            if prime_util::is_prime(i) {
                primes.push(i);
            }
        }

        //now check that the function returns the same list as the brute force algorithm
        let supposed_primes = prime_util::list_of_primes(limit).expect("Not enough primes?!?!");

        // make sure all values are indeed prime
        for prime in supposed_primes.iter() {
            assert_eq!((true,prime), (prime_util::is_prime(*prime),prime));
        }

        // if the sets aren't the same size, check what are missing from the sieve
        // The order should be identical
        if primes.len() > supposed_primes.len() {
            for i in 0..primes.len() {
                assert_eq!(primes[i], supposed_primes[i]);
            }
        }

        assert_eq!(primes.len(), supposed_primes.len());
    }
}
