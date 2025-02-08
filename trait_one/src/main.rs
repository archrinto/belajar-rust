/// trait adalah sifat, interface jika di java
/// 
/// extrenal trait
///     trait yang tempat deklarasinya berada diluar crate kode 
///     yang ditulis. misal std::fmt::Debug
/// 
/// local trait
///     trait yang tempat deklarasinya berada di dalam pacakge / project
///     yg sedang kita kerjakan
/// 
fn main() {
    let circle_one = Circle{raidus: 6};
    println!("{:?}", circle_one);
    println!("Hello, world!");
}

// ini salah satu penerapan trait

struct Circle {
    raidus: i32,
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.raidus)
    }
}