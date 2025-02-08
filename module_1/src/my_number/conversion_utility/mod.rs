// function unwrap digunakan untuk mengambil nilai

pub fn string_to_number(text: String) -> i32 {
    return text.parse::<i32>().unwrap();
}