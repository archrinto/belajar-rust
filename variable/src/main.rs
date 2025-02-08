fn main() {
    let nama_variable = "ini valuenya";
    println!("{}", nama_variable);

    let mut message_number = 1;
    let message1 = "hai";

    println!("message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "dunia";

    println!("message number {}: {}", message_number, message2);

    message_number = 3;
    let message3: i8 = 24;
    println!("message number {1}: {0}", message3, message_number);

    let (var1, var2) = (24, "hello");

    println!("var1: {}", var1);
    println!("var2: {}", var2);

    let (var3, var4): (i8, i8) = (32, 12);

    println!("var3: {}", var3);
    println!("var4: {}", var4);

    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);

    println!("var5: {}", var5);
    println!("var6: {}", var6);

    var6 = 24;

    println!("var6: {}", var6);
    println!("var7: {}", var7);
    // deklarasi variable dengan cara lain
    // let data1 = 24i8;
    let data1 = 24_i8;

    println!("data1: {}", data1);

    // variable shadowing
    let x = 5;
    println!("x: {}", x);

    let x = x + 1;
    println!("x: {}", x);


    // cara deklarasi variable yang tidak digunakan
    let _ = 10;
}
