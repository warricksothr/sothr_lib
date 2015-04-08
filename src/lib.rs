#![feature(core)] //Required to compile 3/31/2015. Test again later when libs have stablized.

pub mod str_util;
pub mod prime_util;

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
}
