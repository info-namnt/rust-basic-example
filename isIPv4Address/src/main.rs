fn main(){
    let input_string  = String::from("01");
    println!("{:?}", solution(input_string) );
}



fn solution(input_string: String) -> bool {
    input_string.split('.').all(|string_num| string_num.parse::<u8>().is_ok())
    && input_string.matches('.').count() == 3
}