// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::num::{Zero, One};
use num::integer::Integer;

/**
    Return the factors of a number

    # Arguments

    * 'number' - The number to find the factor of
    
    # Examples

    ```rust
    use sothr_lib::number_util::factors_for_number;
    assert_eq!(vec![1,3], factors_for_number(3));
    assert_eq!(vec![1,2,4], factors_for_number(4));
    assert_eq!(vec![1,3,5,15], factors_for_number(15));
    ```
*/
pub fn factors_for_number<T: Integer + Clone>(number: T) -> Vec<T> {
    let mut head: Vec<T> = vec![T::one()];
    let mut tail: Vec<T> = vec![number.clone()];
    let mut current = T::one() + T::one();
    let mut limit = number.clone();

    while current < limit {
        if number.clone() % current.clone() == T::zero() {
            limit = number.clone() / current.clone();
            tail.insert(0,limit.clone());
            if limit != current {
                head.push(current.clone());
            }
        }
        current = current + T::one();
    }

    //combine head and tail to get our factors
    for item in tail {
        head.push(item);
    }
    return head;
}

/** 
    Generic factorial function. Accepts any type that implements
    the Integer trait (e.g. i32, u32, BigInt, BigUint, etc). It is up to the
    caller to ensure that the input is >= 0.

    # Examples

    ```rust
    # extern crate num;
    # extern crate sothr_lib;
    # fn main() {
    use num::bigint::{Sign,BigInt,BigUint};
    use sothr_lib::number_util::factorial;
    assert_eq!(362880u32,factorial(9u32));
    assert_eq!(362880i32,factorial(9i32));
    assert_eq!(362880u64,factorial(9u64));
    assert_eq!(362880i64,factorial(9i64));
    assert_eq!(BigInt::new(Sign::Plus,vec![362880]),factorial(BigInt::new(Sign::Plus,vec![9])));
    assert_eq!(BigUint::new(vec![362880]),factorial(BigUint::new(vec![9])));
    # }
    ```
*/
pub fn factorial<T: Integer + Clone>(num: T) -> T {
    if num == T::zero() ||
        num == T::one() {
        T::one()
    } else {
        num.clone() * factorial(num.clone() - T::one())
    }
}

