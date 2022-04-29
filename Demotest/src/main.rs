fn caps(input: &str) -> String {
    input.to_uppercase()
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::result;

    use crate::caps;
    #[test]
    fn test1() {
        let result = caps("Nguyen Nam");
        let expected = String::from("NGUYEN NAM");
        assert_eq!(result, expected, "String should be all upcase")
    }
}