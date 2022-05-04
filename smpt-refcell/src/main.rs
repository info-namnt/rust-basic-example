use std::cell::RefCell;


struct  Channel {
    name: RefCell<String>,
}

fn main() {
    let mychannel = Channel{
        name: RefCell::new("Lap Tring Blockchain".to_owned()),
    };

    {
        let mut a = mychannel.name.borrow_mut();
        *a = "Lap trinh Blockchain".to_owned();
    }

    {
        mychannel.name.replace("Lập trình blockchain".to_owned());
    }

    println!("Mychannel: {:?}", mychannel.name);
}
