// accosiated function adalah fungsi yang terhubung ke sebuah struct
// deklarasinya berada pada block code impl
// pemanggilannya menggunakan notasi path NamaStruct::nama_fungsi

#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

impl LegoSet {
    fn new(code: i32, name: String, category: String, age_minimum: i32) -> LegoSet {
        LegoSet { code, name, category, age_minimum }
    }
}

fn main() {
    // let rough_terrain_crane = LegoSet{
    //     code: 42082,
    //     name: String::from("Rough Terrain Crane"),
    //     category: String::from("Technic"),
    //     age_minimum: 11,
    // };

    let rough_terrain_crane = LegoSet::new(
        42082,
        String::from("Rough Terrain Crane"),
        String::from("Technic"),
        11
    );

    println!("{:#?}", rough_terrain_crane);
}
