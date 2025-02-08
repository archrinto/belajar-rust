mod calculation_spec;
mod two_dimensional;

// perlu import juga traitnya di sini
use crate::calculation_spec::Area;
use crate::calculation_spec::Circumference;

fn main() {
    let circle_one = two_dimensional::Circle{ radius: 10 };
    calculate_and_print_result(String::from("circle_one"), &circle_one);
    calculate_and_print_result2(String::from("circle_one"), &circle_one);
    
    let square_one = two_dimensional::Square{ length: 5};
    calculate_and_print_result(String::from("square_one"), &square_one);
    calculate_and_print_result2(String::from("square_one"), &square_one);
}

// trait sebagai parameter fungsi

fn calculate_and_print_result(name: String, item: &(impl Area + Circumference)) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

// cara penulisan lain trait sebagai parameter fungsi
// menggunakan generic T
fn calculate_and_print_result2<T: Area + Circumference>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

/// contoh lebih kompleks
/// generic T untuk Display + Clone
/// deneric U untuk Clone + Debug
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     // ...
// }

/// Trait dengan where clause
/// Generic T tetap digunakan
fn calculate_and_print_result4<T>(name: String, item: &T) where T: Area + Circumference {
    println!("{} area: {}", name, item.calculate_area());
}

/// Trait juga dapat digunakan untuk return value
fn new_circle(radius: i32) -> impl Area {
    let data = two_dimensional::Circle{
        radius
    };
    data
}

fn new_square(length: i32) -> impl Area + Circumference {
    two_dimensional::Square{
        length
    }
}