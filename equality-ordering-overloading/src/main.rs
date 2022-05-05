use std::ops::Add;

#[derive(Debug)]
struct  Letter(char);


impl Add<Self> for Letter {
    type Output = String;
    fn add(self, input: Self) -> Self::Output {
        format!("{}{}", self.0, input.0)
    }
}
fn main() {
    let c = Letter('h') + Letter('i');
    println!("{}", Letter('h') + Letter('i'));
}
