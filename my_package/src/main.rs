/// Associated types pada trait
/// tipe data yang disematkan pada trait
/// tidak memiliki tipe data konkret saat didefinisikan
/// harus ditentukan data konkretnya ketika trait diimplementasikan

mod shape;
mod circle;
mod square;

// harus diimport di main karena jika tidak
// method .area tidak bisa diakses
use crate::shape::Shape;

fn main() {
    let obj1 = circle::Circle{ radius: 10.0 };
    println!("area of circle: {:.2}", obj1.area());
}
