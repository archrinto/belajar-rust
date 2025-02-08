// ini adalah sebuah path
// std::time::Duration
// std, time, Duration disebut segment, dipisahkan dengan ::
// dalam OS bisanya menggunakan \ atau / sebagai pemisah
// namun di rust menggunakan ::

fn main() {
    // tampilkan intro untuk user agar menginput sebuah pesan
    println!("enter a message:");

    // variabel yang akan menampung inputan user dalam string
    let mut message = std::string::String::new();
    // std adalh standart library di rust
    // string adlh module
    // String adlh struct String
    // new adlh fungsi

    // bisa ditulis
    // String::new()

    // objek reader untuk membaca inputan user
    let stdin_reader = std::io::stdin(); 

    // proses pembacaan inputan user
    let reader_res = stdin_reader.read_line(&mut message);

    // pengecekan apakah ada error dalam pembacaan inputan.
    // jika iya, maka tampilkan error dan hentikan program
    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
        return;
    }

    // tampilkan pesan inputan user
    println!("message: {}", message);

    // pemanggilan mengginakan use
    let stdin_reader = std::io::stdin();

    // ... atau ...

    use std::io;
    let stdin_reader = io::stdin();

    // import dari parent path yang sama
    use std::io::stdin;
    use std::io::stderr;

    // ... atau ...

    use std::io::{stdin, stderr};

    // import semua item dalam path
    use std::io::*;

    // ... adalah ekuivalen dengan ...

    use std::io::{stdin, stderr, stdout, <path lainnya>};
}