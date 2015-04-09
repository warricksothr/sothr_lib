// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
    Determines if a specified number is prime

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
