fn main() {
    let number_a = 3;
    if number_a < 5 {
        println!("number_a adalah di bawah 5");
    }

    let result_a = number_a >= 5;
    if result_a {
        println!("result a adalah diatas atatu sama dengan 5");
    }

    // if untuk menentukan nilai variable
    let result_d = 
        if number_a == 2 {
            true
        } else {
            false
        };
    println!("result_d adalah {result_d}")

    // sama seperti bahasa lainnya
}
