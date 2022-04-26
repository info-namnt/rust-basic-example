fn main() {
    let x = 9;
    let c = test(&x);
    print!("{:?}", c);
}

fn test(x: &i32) -> i32 {
    let y = 10;

    let k = get_ref(&x, &y);
    *k
} 

fn get_ref<'a, 'b: 'a>(param_1 : &'a i32, param_2: &'b i32) -> &'a i32 {
    if (param_1 < param_2) {
        param_1
    } else {
        param_2
    }
}