// inline module tidak perlu menggunakan struktur folder atau file
// dituliskan pada file yang sama

mod utilities;

fn main() {
    let password = format!("zereth mortis {}", utilities::random::string(10));
    println!("raw password: {}", password);

    let hashed = utilities::password::hash(&password);
    println!("hashed password: {}", hashed);

    let is_valid = utilities::password::is_valid(&password, &hashed);
    println!("is password matched? {}", is_valid);
}
