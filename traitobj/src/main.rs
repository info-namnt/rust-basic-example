trait Clicky {
    fn click(&self) -> String;
}


struct  Keyboard;

impl  Clicky for Keyboard {
    fn click(&self) -> String
    {
        String::from("Keyboard Input")
    }
}

struct Mouse;
impl  Clicky for Mouse {
    fn click(&self) -> String {
        String::from("Mouse Input")
    }
    
}

fn main() {
    // println!("Hello, world!");
    let x: Box<dyn Clicky> = Box::new(Keyboard);
    let y: Box<dyn Clicky> = Box::new(Mouse );

    let clicker = vec![x,y];

    for i in clicker {
        if i.click() == "Mouse Input".to_owned(){
            println!("this is mouse");
        } else {
            println!("this is keyboard");

        }
    }


}
