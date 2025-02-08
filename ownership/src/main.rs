// Aturan Ownership
/// 1. setiap nilai/data/value memiliki owner
/// 2. hanya boleh ada 1 owner
/// 3. ketika eksekusi sebuah block scope selesai
///     maka owner dari data-data tsb. akan didrop (dealokasi memory)
///     kecuali dipindah tangankan ke owner diluar scope 
/// 
/// semua tipe data primitif mengikuti perilaku copy semantics
/// untuk tipe selain primitif mengikuti perilaku move semantics
/// 
/// tranfer ownership dapat terjadi melalui
/// 1. assignment
/// 2. return value
/// 3. parameter / argument
/// 
/// semua tipe yang mengadopsi move semantic
/// mengimplement trait Clone
/// ini untuk menghindari perpindahan owner jika diperlukan
/// 
/// atau gunakan saja borrowing 
fn main() {
    println!("Hello, world!");
}
