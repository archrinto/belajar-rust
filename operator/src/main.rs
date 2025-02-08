fn main() {
    // operator aritmatika
    let (num1, num2) = (12, 5);
    let value_addition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_addition);

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);

    let value_mul = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mul);

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);

    let value_mod = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_mod);

    // operator perbandingan
    let number_a = 12;
    let number_b = 24;

    let res1 = number_a == number_b;
    println!("res1: {res1}");

    let res2 = number_a != number_b;
    println!("res2: {res2}");

    let res3 = number_a > number_b;
    println!("res3: {res3}");

    let res4 = number_a < number_b;
    println!("res4: {res4}");

    let res5 = number_a >= number_b;
    println!("res5: {res5}");

    let res6 = number_a <= number_b;
    println!("res6: {res6}");

    // whitespace charater
    println!("AND result \t: {}", false && true);
    println!("OR result \t: {}", false || true);

    // operator bitwise

    // menghasilkan 1 jika kedua bit bernilai 1
    println!("& bitwise \t: {}", 5 & 3);

    // menghasilkan 1 jika salahsatu bit bernilai 1
    println!("| bitwise \t: {}", 5 | 3);

    // menghasilkan 1 jika kedua bit berbeda
    println!("^ bitwise \t: {}", 5 ^ 3);

    // membalik setiap bit
    println!("~ bitwise \t: {}", "");

    // menggeser bit ke kiri sejumlah posisi tertentu
    println!("<< bitwise \t: {}", 5 << 2);

    // menggeser bit ke kanan sejumlah posisi tertentu
    println!(">> bitwise \t: {}", 5 >> 2);
}
