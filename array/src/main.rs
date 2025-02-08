fn main() {
    // semua value harus bertipe data sama
    let mut numbers = [24, 12, 32, 7];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("elemen array ke 0 {data0}");

    let data1 = numbers[1];
    println!("elemen array ke 1 {data1}");

    numbers[1] = 10;
    numbers[3] = -4;

    println!("array {:?}", numbers);


    // deklarasi dengan type interface
    let angka_integer = [24, 12, 32, 7];
    println!("{angka_integer:?}");
    // output: [24, 12, 32, 7]

    let angka_float = [24.2, 12.5, 32.00002, 7.2];
    println!("{angka_float:?}");
    // output: [24.2, 12.5, 32.00002, 7.2]

    // deklarasi dengan manifest typing
    let data_boolean: [bool; 2] = [false, true];
    println!("{data_boolean:?}");
    // output: [false, true]

    let angka_unsigned_integer: [u32; 3] = [24, 0, 12];
    println!("{angka_unsigned_integer:?}");
    // output: [24, 0, 12]

    // deklarasi array dengan notasi [T; N]
    let data_numerik1: [i32; 10] = [2; 10];
    println!("{data_numerik1:?}");
    // output: [2, 2, 2, ...]

    let data_numerik2 = [5; 3];
    println!("{data_numerik2:?}");
    // output: [5, 5, 5]

    let length = data_numerik1.len();
    println!("len: {length}");

    let names: [&str; 4] = ["ani", "budi", "susi", "paijo"];

    for name in names {
        println!("{name}");
    }

    for i in 0..names.len() {
        println!("array index ke-{}: {}", i, names[i]);
    }

    // iterasi dengan tuple
    for (i, name) in names.iter().enumerate() {
        println!("array index ke-{i}: {name}")
    }

    // tidak bisa menambahkan elemen ke array karena akan melebihi kapasitas
    

    // nested array 
    let data_arr = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"],
    ];
    for sub_arr in data_arr {
        for el in sub_arr {
            print!("{el}, ");
        }
        println!();
    }
}
