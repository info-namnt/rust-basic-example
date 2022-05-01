use std::rc::Rc;


struct Car {
    number: String,
}

struct Door {
    vehicle: Rc<Car>
}

fn main() {
    let car =Rc::new (Car { number: "43A 12345".to_string() }) ;

    let left_door = Door {
        vehicle: Rc::clone(&car)
    };

    let right_door = Door {
        vehicle: Rc::clone(&car)
    };
}
