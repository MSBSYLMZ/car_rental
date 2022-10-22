pub mod Car {
    use std::{fmt::Display, io};
    #[derive(Debug)]
    pub struct Car {
        pub brand: String,
        pub model: String,
        pub year: String,
        available: bool,
    }

    impl Display for Car {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let availability = if self.available {
                String::from("Yes")
            } else {
                String::from("Not Currently")
            };
            write!(
                f,
                "Brand: {}, Model: {}, Year: {}, Available: {}",
                self.brand, self.model, self.year, availability
            )
        }
    }

    pub struct Cars {
        enventory: Vec<Car>,
    }

    impl Cars {
        pub fn initialize() -> Cars {
            Self {
                enventory: vec![
                    Car {
                        brand: String::from("Toyota"),
                        model: String::from("Corolla"),
                        year: String::from("2021"),
                        available: true,
                    },
                    Car {
                        brand: String::from("Hyundai"),
                        model: String::from("Accent Era"),
                        year: String::from("2021"),
                        available: true,
                    },
                    Car {
                        brand: String::from("Mercedes"),
                        model: String::from("AMG3"),
                        year: String::from("2021"),
                        available: true,
                    },
                ],
            }
        }

        pub fn list(&self) {
            println!("\n\n================== AVAILABLE CARS =================\n");
            for car in self.enventory.iter() {
                println!("{car}");
            }
            println!("====================================================");
        }

        pub fn rent(&self, car: &str) -> Result<&Car, &'static str> {
            let rental_car = self.enventory.iter().find(|&x| x.brand == car);
            match rental_car {
                Some(V) => Ok(V),
                _ => Err("Can't find the car"),
            }
        }
    }
}
