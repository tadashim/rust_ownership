fn main() {
    let string1 = String::from("Hello,");
    let string2 = String::from(" world!");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
