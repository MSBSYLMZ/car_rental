pub mod car {
    use serde_derive::{Deserialize, Serialize};
    use std::{fmt::Display, env};
    use dotenv::dotenv;

    #[derive(Debug)]
    pub struct Car {
        pub brand: String,
        pub model: String,
        pub year: u32,
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

    #[derive(Serialize, Deserialize, Debug)]
    struct ResponseCar {
        city_mpg: u32,
        class: String,
        combination_mpg: u32,
        cylinders: u32,
        displacement: f32,
        drive: String,
        fuel_type: String,
        highway_mpg: u32,
        make: String,
        model: String,
        transmission: String,
        year: u32,
    }

    impl Cars {
        pub async fn initialize() -> Cars {
            let cars = Self::fetch_cars().await.unwrap();
            Self { enventory: cars }
        }

        pub async fn fetch_cars() -> Result<Vec<Car>, &'static str> {
            dotenv().ok();
            let api_key = env::var("API_KEY").expect("Can't get the API KEY from env");
            let url = "https://api.api-ninjas.com/v1/cars?year=2021&limit=20";

            let client = reqwest::Client::new();
            let res = client
                .get(url)
                .header("X-Api-Key", api_key)
                .send()
                .await
                .unwrap()
                .json::<Vec<ResponseCar>>()
                .await;
            match res {
                Ok(response) => {
                    let response = Self::convert_result_to_car(response);
                    Ok(response)
                }
                _ => Err("Failed to fetch data"),
            }
        }

        fn convert_result_to_car(response: Vec<ResponseCar>) -> Vec<Car> {
            let mut result = Vec::<Car>::with_capacity(response.len());
            for item in response {
                let new_car = Car {
                    brand: item.make.to_uppercase(),
                    model: item.model.to_uppercase(),
                    year: item.year,
                    available: true,
                };
                result.push(new_car);
            }
            result
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
                Some(v) => Ok(v),
                _ => Err("Can't find the car"),
            }
        }
    }
}
