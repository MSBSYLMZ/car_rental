use std::fmt::{Formatter, Display, Error};


#[derive(Debug)]
struct Car {
    name: String,
    model: String,
    year: String,
    available: bool
}

impl Display for Car {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        write!(f, "name: {}, model: {}, available: {}", self.name, self.model, self.available)
    }
}
#[tokio::main]
async fn main() {
    let cars = get_cars();
    println!("Welcome to the Car Rental!");
    println!("Please select the car key that you want to rent");
    println!("{}", cars[0]);
}

fn get_cars() -> Vec<Car>{
    initialize_cars()
}

fn initialize_cars() -> Vec<Car> {
    vec![
        Car { name:String::from("Toyota"), model: String::from("Corolla"), year: String::from("2021"), available: true },
        Car { name:String::from("Hyundai"), model: String::from("Accent Era"), year: String::from("2021"), available: true },
        Car { name:String::from("Mercedes"), model: String::from("AMG3"), year: String::from("2021"), available: true }
    ]
}
