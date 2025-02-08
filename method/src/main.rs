mod models;

fn main() {
    let mut car = models::Car::new(
        String::from("Mitsubisi"),
        String::from("Pajero")
    );

    println!("car {:?}", car);

    let info = car.info();
    println!("info {}", info);

    car.beep(String::from("tet tott!"));

    // ubah nilai prop
    car.set_manufacture_year(2024);
    let info = car.info();
    println!("info {}", info);
}
