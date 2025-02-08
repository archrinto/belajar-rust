/// Pattern Matching

fn main() {
    println!("Hello, world!");
    let value = 6;

    /// jika menggunakan match biasa akan terlihat
    /// seperti ini
    match value {
        Some(1) => println!("one"),
        Some(2) => println!("two"),
        Some(x) => println!("{x} greater than two"),
        None    => println!("none"),
    }

    /// seperti pengguanan if else pada bahasa lain
    /// jika menggunakan pattern maching akan terlihat sepert ini
    /// 
    /// | untuk parttern OR
    /// .. or ..= digunakan untuk IN
    match value {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        6     => println!("six"),
        _     => println!("other number"),
    }

    /// Match guard
    let value = Some(4);
    let message = match value {
        // penambahan kondisi if seperti berikut
        Some(x) if x % 2 == 0 => format!("number {} is even", x),
        Some(x)               => format!("number {} is odd", x),
        None                  => String::new(),
    };

    println!("{message}");

    /// Binding @
    /// Masih kurang paham
    /// 
    /// 
    /// Destruction Assignment
    /// 
    /// Struct destructing
    /// menampung item suatu tipe
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    match p {
        // jika p.y nilainya 0 tampilkan ini
        Point { x, y: 0 } => println!("x axis at {x}"),
        // jika p.x nilainya 0 tampilkan ini
        Point { x: 0, y } => println!("y axis at {y}"),
        // selain itu tampilan ini
        Point { x, y }    => println!("axis: ({x}, {y})")
    }

    /// Enum destcututing
    enum Color {
        Black,
        White,
        Rgb(i32, i32, i32)
    }
    
    let color = Color::Rgb(0, 160, 255);
    // jika color adalah color rgb tampilkan ini
    if let Color::Rgb(r, g, b) = color {
        println!("r: {r}");
        println!("g: {g}");
        println!("b: {b}");
    }
    
    // jika color adalah color rgb tampilkan ini
    match color {
        Color::Rgb(r, g, b) => println!("r: {r}, g: {g}, b: {b}"),
        _                   => println!("other color")
    }

    /// variable _ digunakan untuk menampung value yang tidak digunakan
    let numbers = (2, 4, 32);

    let (_, second, _) = numbers;
    println!("second number: {second}");

    /// Operator ..
    /// untuk mengambil range
    let (first, .., last) = numbers;
    println!("first number: {first}");
    println!("last number: {last}");

    let (first, ..) = numbers;
    println!("first number: {first}");

    let (.., last) = numbers;
    println!("last number: {last}");
}
