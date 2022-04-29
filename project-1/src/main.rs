use std::{io, vec, collections::HashMap};

// CRUD APPLICATION

// Giúp tham chiếu và nhân bản thêm 1 struct khác
#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: i32
}

pub struct Students {
    class: Vec<Student>
}



mod manager {
    use crate::{Student, Students, get_input, get_int};

    pub fn add_student(students:&mut Students) {
        println!("Enter the name of student:");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };


        let age = match get_int() {
            Some(input) => input,
            None => return
        };

        let student = Student { name, age};

        students.add(student)
    }

    pub fn view_all(students:&Students) {
        for student in students.view_all() {
            println!("{:?}", student);
        }
    }

}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add(&mut self, student: Student) {
        self.class.insert(student)
    }

    fn view_all(&self) -> Vec<&Student> {
        self.class.iter().collect()
    }

}



fn get_int() -> Option<i32> {
    println!("Enter age of student:");

    let input = match get_input() {
        Some(input) => input,
        None => return None
    };

    let parse_input: Result<i32, _> = input.parse();

    match parse_input {
        Ok(input) => Some(input),
        Err(_) => None     
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again");
    }

    let input = buffer.trim().to_owned();

    if input == "" {
        None    
    } else {
        Some(input)
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent
}

impl Manager {
    fn show() {
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please Enter Your Choice");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),            
            "3" => Some(Manager::EditStudent),            
            "4" => Some(Manager::DeleteStudent),            
            _ => None
        }
    } 
}
fn main() {
    let mut students = Students::new();
    // Nếu nhập phù hợp thì sẽ lập lại, còn sai sẽ out 
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view_all(&students),
            Some(Manager::EditStudent) => (),
            Some(Manager::DeleteStudent) => (),
            None => return
            
        }
    }
}
