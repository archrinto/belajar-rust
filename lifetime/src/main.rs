/// Lifetime
/// digunakan untuk memonitor umur dari reference agar 
/// dianggap valid
/// 
/// Ini perlu diperhatikan jika berurusan dengan tipe 
/// data reference atau pointer
/// 
/// Rust menggunakan Lifetime Elision
/// 
let r;
{
    let x = 5;
    r = &x;
}
println!("r: {}", r);
/// kode diatas akan error karena x akan didealokasi ketika
/// block selesai.
/// 
/// Jika mengembalikan value dari sebuah function atau method
/// jangang mengembalikan pointer yg ownernya ad di blcok fungsi
/// tersebut. karena dapat menyebabkan error. Owner dari datanya
/// sudah tidak lagi di memory karena telah didealokasi
/// 

fn main() {
}

// ini error
let m: &String = get_message();
    
fn get_message() -> &String {
    let message = String::from("darkspear is better than zandalari");
    &message
}

// ganti kode berikut ...
fn get_message() -> &String {
    let message = String::from("darkspear is better than zandalari");
    message
}

// ini tidak
// ... menjadi ...
fn get_message() -> String {
    let message = String::from("darkspear is better than zandalari");
    message
}

/// Annotasi Lifetime
/// 'nama_lifetime
/// 
/// Berfungsi untuk memberitahu compiler agar reference tidak langsung
/// didealokasi seletalh block selesai
/// 
/// deklarasinya mirip dengan block label
&i32        // => tipe data reference i32
&'a i32     // => tipe data reference i32 dengan lifetime 'a
&'a mut i32 // => tipe data mutable reference i32 dengan lifetime 'a

// deklarasi annotasi lifetime pada return
fn get_number<'my_lifetime>() -> &'my_lifetime i32 {
    &13
}

// deklarasi annotasi lifetime pada parameter
fn do_something_v5<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'b str {
    y
}

// deklarasi annotasi lifetime pada struct
struct Book<'abc> {
    title: &'abc str,
    description: &'abc str,
}