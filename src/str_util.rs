/**
    Converts a vector of types that have the ToString trait into 
    a pretty string
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
