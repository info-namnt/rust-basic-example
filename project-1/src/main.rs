use std::io;

// CRUD APPLICATION
enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    io::stdin().read_line(buf)
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
            "2" => Some(Manager::AddStudent),            
            "3" => Some(Manager::AddStudent),            
            "4" => Some(Manager::AddStudent),            
            _ => None
        }
    } 
}
fn main() {
    Manager::show();
}
