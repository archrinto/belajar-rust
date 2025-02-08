// Pointer
/// pointer adalah alamat memory, hanya alamat, tidak valunya
/// pointer ditandai dengan karakter & 
/// contoh &i32
/// 
/// Reference adalah pointer dari sebuah variable
/// semua variable dapat diambil pointernya
let number: i32 = 24;
println!("value: {:?}", number);

// ini digunakan untuk menampilkan pointernya (alamat memory)
let pointer_number: &i32 = &number;
println!("pointer: {:p}", pointer_number);

// Dereference
/// cara mengambil nilai sebenarnya dari sebuah pointer
/// dengan menggunakan karakter *
/// pada variable pointer

let underlying_value = *pointer_number;
println!("value: {:}", underlying_value);


/// Jika tedapat deklarasi seperti ini
/// maka rust akan mengalokasikan memory lagi untuk (duplikasi)
/// valirable number_two
let number_one = 24;
let number_two = number_one;

/// karena semua variable primitif mengadopsi copy semantic
/// 
/// Jika deklarasi seperti berikut ini maka number_two akan menggunakan
/// pointer number_one untuk menjadi reference
/// tidak terjadi duplikasi
let number_one = 24;
let number_two = &number_one;

// pada dasarnya jika kita menggunakan reference
// kita meminjam (borrowing) data dari ownernya

fn main() {
    println!("Hello, world!");
}
