// tipe alias
// pemberian nama baru ke suatu tipe data

// contoh tipe data Inch adalah alias dari u64
type Inch = u64;

// contoh tipe data Coordinate dari tipe data Point
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
type Coordinate = Point;

// Casting
// mengubah tipe data tertentu ke tipe data lain yg masih compatibel

let height Inch = 6;
let heught_64 = height as u64;

let p = Point{ x: 0, y: 10 };
let mut q: Coordinate = p as Coordinate;

let r: Point = q as Point;

// operasi assignment dan type casting pada custom type struct
// membuat owenernya berpindah (move semantics)

fn main() {
    println!("Hello, world!");
}
