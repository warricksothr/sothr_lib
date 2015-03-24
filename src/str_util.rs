///
/// Converts a vector of types that have the ToString trait into 
/// a pretty string
///
/// # Arguments
///
/// * 'vec' - The vector you'd like a pretty string for
///
/// # Examples
///
/// ```rust
/// let v = vec![1,2,3]
/// string_for_vector(v) // prints "1,2,3"
/// ```
///
pub fn string_for_vector<T: ToString>(vec: Vec<T>) -> String {
    let mut s = String::new();
    for x in &vec {
        s.push_str(&x.to_string());
        s.push(',');
    }
    s.pop();
    s
}
