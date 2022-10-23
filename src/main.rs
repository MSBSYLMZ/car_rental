use std::io;

use car_rental::car::car::Cars;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let cars = Cars::initialize().await;
    cars.list();
    loop {
        let mut choice = String::new();
        println!("Please select the car brand that you want to rent.");
        io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();
        match cars.rent(choice) {
            Ok(v) => {
                println!("You have succesfully rent this car:");
                println!("Brand: {}, Model: {}, Year: {}", v.brand, v.model, v.year);
                break;
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
