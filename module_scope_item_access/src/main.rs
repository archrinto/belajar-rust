fn my_func() {
    println!("calling my_func()")
}

mod my_mod {
    pub fn my_func() {
        println!("calling my_mod::my_func()");
    }   

    // kita pindah ke submod
    // pub fn run_the_app() {
    //     println!("calling my_run_the_app");
    //     my_func();
    //     self::my_func();
    //     // untuk memanggil function di crate root
    //     crate::my_func();
    // }

    pub mod my_submod {
        pub fn my_func() {
            println!("calling my_mod::my_submod::my_func()");
        }

        pub fn run_the_app() {
            println!("calling my_mod::my_submod::run_the_app");
            // untuk memanggil function di crate root
            crate::my_func();
            super::super::my_func();
            // untuk memanggil function di parent mod
            super::my_func();
            // untuk memanggil functio di mod ini
            self::my_func();
        }
    }
}

fn main() {
    my_mod::my_submod::run_the_app();
}
