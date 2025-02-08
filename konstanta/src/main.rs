// memiliki nilai memory yang fix
static NUMBER: i32 = 18;

fn main() {
    // sedangkan const tidak, terjadi proses copy value tiap digunakan
    const LABEL: &str = "nilai pi adalah";
    const PI: f32 = 22.0/7.0;
    println!("{} {:.3}", LABEL, PI);

    println!("{}", NUMBER);
}
