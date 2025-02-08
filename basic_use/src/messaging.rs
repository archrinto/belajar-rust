// use digunakan untuk melakukan export dari private module
// bisa juga dengan memeberi alias

pub use self::sub_module::say_hello_message;
pub use self::sub_module::say_hello_message as say_hello;

mod sub_module {

    pub fn say_hello_message() {
        println!("hello rust")
    }
}
