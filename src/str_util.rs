/**
    Converts a vector of types that support to_string  into a string
*/
pub fn string_for_vector<T: ToString>(vec: Vec<T>) -> String {
    let mut s = String::new();
    for i in 0..vec.len() {
        s.push_str(&vec[i].to_string());
        if i + 1 < vec.len() {
            s.push(',');
        }
    }
    return s;
}
