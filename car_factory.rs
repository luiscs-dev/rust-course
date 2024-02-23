
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}


fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car{ color: color, transmission: transmission, convertible: convertible, mileage: 0}
}


fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
}
