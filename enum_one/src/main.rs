// enum ditulis dengan keyword enum

enum NamaEnum {
    NilaiEnum1,
    Nilai2,
    NilaiEnumKe3,
}

// definisi konstanta
const SuperheroSuperman: &str = "superman";
const SuperheroOmniMan: &str = "omniman";
const SuperheroHomelander: &str = "homelander";
const SuperheroHyperion: &str = "hyperion";

// definisi enum
enum Superhero {
    Superman,
    OmniMan,
    Homelander,
    Hyperion,
}

enum Food {
    PenyetanTerangBulan,
    PizzaNanas,
    EsKrimIkanMujaer,
    MiGorengKuah,
}

fn main() {
    let makanan_favorit: Food = Food::PenyetanTerangBulan;
    
    // keyword match digunakan untuk mengecek value dengan enum
    match makanan_favorit {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        },
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineaple on top of pizza");
        },
        Food::EsKrimIkanMujaer => {
            println!("I don't know what to say");
        },
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        }
    }
}
