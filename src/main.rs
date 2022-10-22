use std::io;

use car_rental::Car::Car::Cars;

#[tokio::main]
async fn main() {
    run();
}

fn run() {
    let cars = Cars::initialize();
    cars.list();
    loop {
        let mut choice = String::new();
        println!("Please select the car brand that you want to rent.");
        io::stdin()
        .read_line(&mut choice)
        .unwrap();

        let choice = choice.trim();
        
        match cars.rent(choice) {
            Ok(V) => {
                println!("You have succesfully rent this car:");
                println!("Brand: {}, Model: {}, Year: {}", V.brand, V.model, V.year);
                break;
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
    
}
