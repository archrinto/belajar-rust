// SEMUA fungsi pada rust mengembalikan nilai
// jika pada fungsi yang returnnya tidak didefinisikan
// makan nilai balikannya adalah
// tuple kosong ()


fn greet() {
    println!("hello world");
}

fn main() {
    println!("Hello, ndan!");
    greet();
    greet_custome_message("paijo", "apa kabar?");
    let result = calculate_box_volume1(5, 5, 10);
    println!("result: {result}");
    let result2 = calculate_box_volume2(5, 5, 10);
    println!("result: {result2}");
    println!("result: {}", get_score_message(100.0));
}

// penamaan fungsi dengan snake case
// tidak mengenal urutan pendefinisian

// parameter adalah variable yang didefinisikan di fungsi
// sedangkan argumen adalah nilai yang disisipkan pada parameter
// saat pemanggilan fungsi

fn greet_custome_message(name: &str, message: &str) {
    println!("hi {name}, {message}")
}

// return value 
// definisikan tipe return value menggunakan tanda ->
// pada fungsi gunakan keyword return
fn calculate_box_volume1(width: i32, height: i32, length: i32) -> i32 {
    return width * height * length;
}

// Unik!
// atau tanpa return, namun dengan deklarsi line terakhir tanpa ;
fn calculate_box_volume2(width: i32, height: i32, length: i32) -> i32 {
    // let volume = width * height * length;
    // volume
    width * height * length
}


// macro format!
// digunkan untuk formating string
// let message3 = format!("the box volume is {}", res3);
// println!("{}", message3.as_str());
// .as_str() akan mengambil value dari String menjadi &str

// conditional return
fn get_score_message(score: f32) -> &'static str {
    if score == 100.0 {
        return "you got a perfect score!"
    }
    
    if score > 76.0 {
        return "congrats, you passed the exam!"
    }

    "your score is below the passing grade"
}
