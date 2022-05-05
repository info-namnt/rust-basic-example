#[derive(Debug)]
enum ClassType {
    Good,
    Bad
}
#[derive(Debug)]
struct Class {
    name: String,
    kind: ClassType
}

fn main() {
    let classA = Class {
        name: String::from("A15"),
        kind: ClassType::Good
    };

    println!("{:?}", classA);
    println!("{:?}", classA);
}
