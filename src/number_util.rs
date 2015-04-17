// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
pub fn factors_for_number(number: u64) -> Vec<u64> {
    let mut head: Vec<u64> = vec![1u64];
    let mut tail: Vec<u64> = vec![number];
    let mut current = 2u64;
    let mut limit = number;

    while current < limit {
        if number % current == 0 {
            limit = number/current;
            tail.insert(0,limit);
            if limit != current {
                head.push(current);
            }
        }
        current += 1;
    }

    //combine head and tail to get our factors
    for i in 0..tail.len() {
        head.push(tail[i]);
    }
    return head;
}
