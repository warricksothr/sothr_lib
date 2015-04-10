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
  let mut sieve: Vec<bool> = Vec::with_capacity(end/2);
  let mut i: usize = 0;
  while i < end {
    sieve.push(true);
    i += 1;
  }

  let f_look_until = look_until as f64;
  let estimate_of_primes = ((f_look_until / f_look_until.ln()) * (1f64 + (1.2762 / f_look_until.ln()))) as usize;
  let mut primes: Vec<u64> = Vec::with_capacity(estimate_of_primes);
  primes.push(2u64);

  let limit = (((look_until/2) as f64).sqrt() as usize) + 1;

  // in this lineup index 0 == 3
  // therefore index 1 == 5, 2 == 7, 3 == 9, etc...
  for i in 0..end/2 {
    if sieve[i] {
      primes.push((i+i+3) as u64);
      if i < limit {
        // selectively sieve out non primes. We're only concerned with odds as
        // we've already removed even values from the list.
        // i = 0, therefore i represents 3
        // i+i+3 = 3
        // simple proofs of odd counts
        // 0 1 2 3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23
        //[3,5,7,9,11,13,15,17,19,21,23,25,27,29,31,33,35,37,39,41,43,45,47,49]
        //0,3,6,9,12 = <3>,9,15,21,27
        //1,6,11,16 = <5>,15,25,35
        //2,9,16,23 = <7>,21,35,49
        for j in range_step_inclusive(i+i+3, end-1, i+i+3) {
          sieve[j] = false;
        }
      }
    }
  }
  Some(primes)
}

/**
    Find a specified number of primes.

    # Arguments

    * 'look_until' - The end of the range to look for primes within

    # Examples

    ```rust
    use sothr_lib::prime_util::find_number_of_primes;
    let prime_list = vec![2,3,5,7,11];
    assert_eq!(prime_list, find_number_of_primes(5).expect("No Primes?!?!?"));
    ```
*/
pub fn find_number_of_primes(num_of_primes: usize) -> Option<Vec<u64>> {
    // very slow way to find primes
    // TODO: Use a more efficient algorithm in the future
    
    if num_of_primes == 0 {
        return None;
    }

    let mut primes: Vec<u64> = Vec::with_capacity(num_of_primes);
    primes.push(2u64);

    let mut num = 3u64;
    loop {
        let mut is_prime = true;
        for prime in primes.iter() {
            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime { primes.push(num); }
        if primes.len() >= num_of_primes { break; }
        num += 1;
    }
    
    Some(primes)
}
