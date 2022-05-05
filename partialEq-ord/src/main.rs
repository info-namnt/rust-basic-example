use std::cmp::{PartialOrd, Ordering};


#[derive(PartialEq, PartialOrd)]
enum Employee {
    Marketer,
    Developer,
}

#[derive(PartialEq)]
struct User {
    id   :i32,
    name: String,
}


impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if self.name < other.name {
        //     Some(Ordering::Less)
        // } else if self.name > other.name {
        //     Some(Ordering::Greater)
        // } else {
        //     Some(Ordering::Equal)
        // }
        Some(self.name.cmp(&other.name))
    }
}

fn main() {
    // let a = Employee::Marketer;
    // let b = Employee::Developer;


    let a = User{
        id: 1,
        name: "A".to_owned(),
    };

    let b = User{
        id: 2,
        name: "B".to_owned(),
    };

    // if a > b {
    //     println!("true");
    // } else {
    //     println!("false");
    // }

    let  c= a.partial_cmp(&b);

    println!("{:?}", c);
}
