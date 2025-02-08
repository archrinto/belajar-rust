/// Borrowing
/// meminjam data milik owner, tujuannya agar dapat mengakses
/// value tanpa memindah ownernya. setelelah selesai
/// dikembalikan
/// 
/// Level akses read only (immutable) menggunakan reference &
let msg_3 = String::from("hello rust");
let msg_4 = &msg_3; // <----- borrow operation

println!("{:?}", msg_4); // output => hello rust
println!("{:?}", msg_3); // output => hello rust

// level akses mutable menggunakan reference &mut
let mut msg_3 = String::from("hello");
let msg_4 = &mut msg_3; // <----- mutable borrow operation

*msg_4 = String::from("hello rust");

println!("{:?}", msg_4); // output => hello rust
println!("{:?}", msg_3); // output => hello rust

/// aturan borrowing
/// 1. dalam waktu yang sama hanya boleh ada satu mutable reference
/// 2. tidak boleh ada mutable reference dengan immnutable reference
///     dalam waktu yang sama
/// 2. reference harus selalu valid
/// 
/// yang dimaksud diwaktu yang sama adalah dalam satu scope yg sama

fn main() {
    println!("Hello, world!");
}
