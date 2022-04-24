fn main() {
    let result = solution(String::from("abcd"));
    println!("{:?}", result);
}

fn solution(input_string: String) -> bool {
    let rev_str : String = input_string.chars().rev().collect();
    return rev_str == input_string;
   }
   