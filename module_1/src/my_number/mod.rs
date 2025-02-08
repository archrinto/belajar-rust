// function unwrap digunakan untuk mengambil nilai

// pub fn string_to_number(text: String) -> i32 {
//     return text.parse::<i32>().unwrap();
// }


// pub mod conversion_utility;

// selain itu juga bisa menggunakan path
#[path = "conversion.rs"]
pub mod conversion_utility;

pub fn is_odd_number(number: i32) -> bool {
    number % 2 == 1
}