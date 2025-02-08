/// generic
/// digunakan untuk membuat fleksibelitas tipe data
/// digunakan pada parameter atau return value

fn do_something<T>(arg1: i32, arg2: T) {

}

/// pada contoh kode diatas adalah contoh deklarsi generic
/// bisa digunakan seperti pada kode berikut
do_something::<bool>(24, false);
/// jadi pada argument tipe data dari parameternya 
/// harus sama untuk tipe datanya
do_something(24, false);
/// kedua kode tsb. ekuivalen

/// tidak ada batasan deklarasi generic
/// contoh pada kode berikut terdapat dua parameter generic

fn do_something_v2<R, T>(arg1: R, arg2: T) {
    // ...
}

/// pada dasarnya generic tidak bisa dilakukan operasi apapun
/// jadi supaya bisa digunakan untuk operasi-operasi tertentu
/// maka perlu mendeklarasikan trait terkait
fn print_x_times<T: std::fmt::Debug>(data: T, x: i32) {
    for _ in 0..x {
        println!("{:?}", data);
    }
}

// fn nama_fungsi<T: Trait1 + Trait2 + ...>(arg1 ...)

fn do_something<T, U, V>(arg1: T, arg2: U, arg3: V) 
where
    T: some::traits:TraitA,
    U: some::traits:TraitB + some::traits:TraitC + some::traits:TraitD,
    V: some::traits:TraitA + some::traits:TraitD,
{
    // do something
}

// deklarasi generic pada struc
struct Point<T> {
    x: T,
    y: T
}
let num: Point<i32> = Point { x: 50, y: 80};

// deklarasi generic pada method
impl<T> Square<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// deklarasi generic pada enum
enum Kendaraan<T> {
    Skateboard,
    SepedaPancal,
    Gledekan(T),
}

fn main() {
    println!("Hello, world!");
}
