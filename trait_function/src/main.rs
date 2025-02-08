/// Trait -> Function
/// - Fn
/// - FnMut
/// - FnOnce

/// mirip lambda pada python

/// Fn dapat dipanggil berkali kali
/// asalkan di dalam closure tsb. tidak ada operasi mutable
fn do_something_with_number_v1<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32, // <----- Fn digunakan
{
    // Statement `f(n)` bisa dipanggil berkali-kali
    return f(n);
}

/// FnMut dapat dipanggil berkali-kali
/// dan bisa memutate data suatu variable yang berada
/// di luar scope block
fn do_something_with_number_v2<F>(n: i32, mut f: F)
where
    F: FnMut(i32), // <----- FnMut digunakan
{
    // Statement `f(n)` berisi kode yang mengubah isi variavel `number` (mutable).
    // `f(n)` bisa dipanggil berkali-kali
    f(n);
}

/// FnOnce dapat dipanggil sekali saja
/// dan bisa memutate sutau data atau tidak
/// jika dipanggil beberapa kali akan error
fn do_something_with_number_v3<F>(n: i32, f: F)
where
    F: FnOnce(i32), // <----- FnOnce digunakan
{
    f(n);
    f(n); // ini kan error karena dipanggil > 1x
}

fn main() {
    let result = do_something_with_number_v1(13, |d: i32| d * 2);
    println!("result: {result}");


    // Trait FnMut
    let mut number = 1;
    do_something_with_number_v2(14, |x| number += x);
    println!("number: {number}");
}

