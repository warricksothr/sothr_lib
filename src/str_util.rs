// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::order::equals;

/**
  Converts a vector of types that have the ToString trait into 
  a pretty string

  # Arguments

  * 'vec' - The vector you'd like a pretty string for

  # Examples

  ```rust
  use sothr_lib::str_util::string_for_vector;
  let v = vec![1,2,3];
  assert_eq!("1,2,3", string_for_vector(v)); // prints "1,2,3"
  ```
*/
pub fn string_for_vector<T: ToString>(vec: Vec<T>) -> String {
    let mut s = String::new();
    for x in &vec {
        s.push_str(&x.to_string());
        s.push(',');
    }
    s.pop();
    s
}

/**
  Checks if a string is a palindrome

  # Arguments

  * 'str' - The string slice to check for palindromicness

  # Examples

  ```rust
  use sothr_lib::str_util::is_palindrome;
  let s = "tenet";
  assert_eq!(true, is_palindrome(s)) // returns true
  ```
*/
pub fn is_palindrome(str: &str) -> bool {
  let bytes = str.as_bytes();
  let iter = bytes.iter();
  let iter_rev = bytes.iter();
  let n = bytes.len() / 2;
  equals(iter.take(n), iter_rev.rev().take(n))
}

/**
  Replaces certain characters within a string
  
  # Arguments
  
  * 'str' - The borrowed string slice to convert into a new string with the replaced characters
  * 'to_replace' - The character to replace from the string
  * 'replace_with' - The character to replace it with

  # Examples

  ```rust
  use sothr_lib::str_util::replace_char;
  let s = "test";
  assert_eq!("tast",replace_char(s,'e','a'));
  ```

*/
pub fn replace_char(str: &str, to_replace: char, replace_with: char) -> String {
  let mut result = String::new();
  for ch in str.chars() {
    if ch == to_replace {
      result.push(replace_with);
    } else {
      result.push(ch);
    }
  }
  result
}
