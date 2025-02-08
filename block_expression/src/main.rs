// block expression adalah kode program yg ditulis diantara { dan }
// Block biasa diterapkan untuk isolasi sebuah proses yang tidak perlu di-reuse. 
let x = 24;

{
    let y = 12;
    let z = x + y;
};

println!("z: {}", z); // <------ error

fn main() {
    println!("Hello, world!");
}
