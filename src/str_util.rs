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
  let n = "iamnotapalindrome";
  assert_eq!(true, is_palindrome(s)); // returns true
  assert_eq!(false, is_palindrome(n)); //return false
  ```
*/
pub fn is_palindrome(str: &str) -> bool {
  let iter =str.chars();
  let iter_rev = str.chars().rev();
  let n = str.len() / 2;
  equals(iter.take(n), iter_rev.take(n))
}

/**
  Replaces certain characters within a string and returns a new string. 
  The original string is untouched
  
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

/**
  Replaces a string within the provided string and returns a new string.

  # Arguments

  * 'str' - The borrowed string
  * 'to_replace' - The substring to replace
  * 'replace_with' - The string to replace the substring with

  # Examples

  ```rust
  use sothr_lib::str_util::replace_string;
  let s = "This is the string to replace stuff in";
  assert_eq!("This is the cow to replace stuff in",replace_string(s, "string", "cow"));
  assert_eq!("This is the string to replace stuff in",replace_string(s, "cow", "string"));
  assert_eq!("Thisisthestringtoreplacestuffin",replace_string(s, " ", ""));
  ```

*/
pub fn replace_string(str: &str, to_replace: &str, replace_with: &str) -> String {
  let mut result = String::new();
  let substr_len = to_replace.len();
  let substr_chars: Vec<char> = to_replace.chars().collect();
  let str_chars: Vec<char> = str.chars().collect();
  // walking over the string slice
  let mut i = 0;
  while i < str_chars.len() {
    let mut found_count = 0;
    let mut found = false;
    for j in 0..substr_len {
      // Don't go out of bounds during the check
      if i+j >= str_chars.len() {
        break;
      // Check for replacement
      } else if str_chars[i+j] == substr_chars[j] {
        found_count += 1;
        if found_count == substr_len {
          found = true;
          break;
        }
      }
    }
    if found {
      for ch in replace_with.chars() {
        result.push(ch);
      }
      i += substr_len;
    } else {
      result.push(str_chars[i]);
      i += 1;
    }
  }
  result
}
