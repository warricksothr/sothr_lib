// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::range_step_inclusive;

/**
    Determines if a specified number is prime.

    # Arguments 

    * 'num' - The number to check for primality

    # Examples 

    ```rust
    use sothr_lib::prime_util::is_prime;
    assert_eq!(true, is_prime(1));
    assert_eq!(true, is_prime(5));
    assert_eq!(false, is_prime(6));
    assert_eq!(true, is_prime(6857));
    assert_eq!(false, is_prime(75031));
    ```
*/
pub fn is_prime(num: u64) -> bool {
    // 1-3 are prime numbers
    if num <= 3 {
        return true;
    }
    
    // numbers divisible by 2 and 3, but greater than 3 are not prime
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    // Check every number between i=5 and the square of i than is less than the
    // desired number
    let mut i = 5;
    while i*i < num {
    	// If the number is evenly divisible by i or i+2 it is not prime
        if num % i == 0 || num % (i+2) == 0 {
            return false;
        }
        // Increment by 6 because it is the product of 2 and 3. Therefore of 
        // the numbers between 5 and 11 (i and i+6) only 5 and 7 could be prime
        // values. 6, 8, 9 and 10 are products or sums of 2 and 3.
        i += 6;
    }
    // If this far, then the number is definitively prime
    true
}

/**
  Generate a list of primes between 0 and the end value.

  # Arguments

  * 'look_until' - The end of the range to look for primes within

  # Examples

  ```rust
  use sothr_lib::prime_util::list_of_primes;
  let prime_list = vec![2,3,5,7,11];
  assert_eq!(prime_list, list_of_primes(12).expect("No Primes?!?!?"));
  ```
*/
pub fn list_of_primes(look_until: u64) -> Option<Vec<u64>> {
  if look_until < 2u64 {
    return None
  }

  let end = look_until as usize;

  //prepopulate the sieve
  let mut sieve: Vec<bool> = Vec::with_capacity(end);
  let mut i: usize = 0;
  while i < end {
    sieve.push(true);
    i += 1;
  }

  let f_look_until = look_until as f64;
  let estimate_of_primes = ((f_look_until / f_look_until.ln()) * (1f64 + (1.2762 / f_look_until.ln()))) as usize;
  let mut primes: Vec<u64> = Vec::with_capacity(estimate_of_primes);
  primes.push(2u64);

  let limit = (f_look_until.sqrt() as usize) + 1;

  for i in range_step_inclusive(3usize, end-1, 2usize) {
    if sieve[i] {
      primes.push(i as u64);
      if i < limit {
        for j in range_step_inclusive(i*i, end-1, i) {
          sieve[j] = false;
        }
      }
    }
  }

  Some(primes)
}