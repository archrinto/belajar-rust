fn main() {
    // tipe data range 0..5 

    // mencetak 0 sampai 4
    for i in 0..5 {
        println!("{i}");
    }

    // mencetak 0 sampai 5
    for i in 0..=5 {
        println!("{i}");
    }

    // perulangan menggunakan label
    'perulangan: for i in 0..=5 {
        if i > 3 {
            println!("berhenti nih gaes!");
            break 'perulangan;
        }
    }

    let array = ["jason", "aaa", "grayin", "drake", "damian"];
    for name in array {
        println!("{name}")
    }
}
