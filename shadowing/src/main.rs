// shadowing
// bisa dilihat pada kode dibawah
// variablenya menggunakan nama yang sama
// dan tipe data yang beda
// pada rust, ini bukan menggati nilai di memory
// semua variable ini terimpan di memory
// hanya ketika dipanggil nama varibale tersebut
// tertutup oleh variable terbarunya
// sehingga variable yang seblumnya tidak bisa diakses

fn main() {
    let some_data = "Hello";
    println!("{}", some_data);
    // output => Hello

    let some_data = 12;
    println!("{}", some_data);
    // output => 12

    let some_data = "Rust!";
    println!("{}", some_data);
    // output => Rust!

    let mut some_data = false;
    some_data = true;
    println!("{}", some_data);
    // output => true

    let some_data = 3.14;
    println!("{}", some_data);
    // output => 3.14


    let name = "Orgrim Doomhammer";
    let mut race = "Orc";
    let mut number = 27;

    println!("{}\t {}\t {}", name, race, number);

    {
        let name = "Sylvanas Windrunner";
        race = "Undead";
        let number = 24;
        
        println!("{}\t {}\t {}", name, race, number);
    }
        
    println!("{}\t {}\t {}", name, race, number);
}