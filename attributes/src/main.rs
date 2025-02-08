/// Attributes
/// 
/// untuk crate, module, module item
/// 
/// Outer attributes
/// dituliskan tepat sebelum target
/// 
/// #[attribute = "value"]
/// #[attribute(key = "value")]
/// #[attribute(value)]
/// 
#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

/// attribute derive digunakan untuk mempermudah implementasi
/// trait ke tipe data
#[derive(PartialEq, Debug)]
enum Superhero {
    Superman,
    OmniMan,
    Hyperion,
}

/// PartialEq adalah trit yang diperlukan untuk seleksi kondisi if
/// Debug adalah trait yang diperlukan untuk bisa cetak di console

impl std::fmt::Display for Superhero {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Attribute cfg / configuration
/// digunakan untuk operasi-operasi yang berhubungan dengan target
/// arsitektur hardware / prosesor. Seperti kondisional compilation
/// ketika os linux dan lainya
#[cfg(target_os = "linux")]
mod util {

    pub fn say_hello() {
        println!("hello (from linux)")
    }
}

#[cfg(target_os = "windows")]
mod util {

    pub fn say_hello() {
        println!("hello (from windows)")
    }

    pub fn say_something() {
        println!("how are you")
    }
}
/// - target_os: windows, linux, macos, android, dll
/// - target_arch: x86, x86_64 dll

/// macro cfg!
fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("hello linux. from attribute cfg")
    }

    #[cfg(target_os = "windows")]
    {
        println!("hello windows. from attribute cfg")
    }

    // atau bisa pakai kondisional statement seperti ini
    if cfg!(target_os = "linux") {
        println!("hello linux. from macro cfg!()");
    } else if cfg!(target_os = "windows") {
        println!("hello windows. from macro cfg!()");
    }
}

/// - debug_assertations. 
/// tergantung mode compilasi apakah debug atau release 
fn main() {
    #[cfg(debug_assertions)]
    {
        println!("debug mode. from attribute cfg")
    }

    #[cfg(not(debug_assertions))]
    {
        println!("release mode. from attribute cfg")
    }

    if cfg!(debug_assertions) {
        println!("debug mode. from macro cfg!()");
    } else  {
        println!("release mode. from macro cfg!()");
    }
}


/// Attribute linting & diagnostic
/// digunakan untuk mengoverride default lingint milik rust

#[allow(unused_imports)]
use std::fmt::Display;

#[allow(unused_variables)]
let name = "noval agung";

#[warn(missing_docs)]
pub fn undocumented_too() -> i32 { 2 }

/// attribute module
/// misal untuk custom import module

mod util2;

#[path = "util3_mymodule.rs"]
mod util3;

fn main() {
    println!("Hello, world!");
    let value: Superhero = Superhero::Superman;

    if value == Superhero::Superman {
        println!("hello superman!");
    }
}
