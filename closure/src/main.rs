/// Closure
/// fungsi anonim
/// fungsi yang disimpan dalam variable
/// 
let calculate_circle_volume_v2 = |e: f64| -> f64 {
    const PI: f64 = 3.14;
    let volume = 4.0 / 3.0 * PI * e.powi(3);
    volume
};

/// deklarasi menggunakan | |
/// bisa punya parameter atau tidak
/// 
// closure dengan 2 parameter tanpa return value
let do_something_v1 = | a: i32, b: String | {
    // ...
};

// closure dengan 2 parameter dan return value bertipe tuple
let do_something_v2 = | a: i32, b: String | -> (i32, bool) {
    // ...
};

// closure tanpa parameter dan return value bertipe Vec<String>
let do_something_v3 = || -> Vec<String> {
    // ...
};

// closure tanpa parameter dan tanpa return value
let do_something_v4 = || {
    // ...
};


fn pow_v1(x: i32) -> i32 {
    x.pow(2)
}

let pow_v2 = |x: i32| -> i32 {
    x.pow(2)
};

let pow_v3 = |x: i32| {
    x.pow(2)
};

let pow_v4 = |x: i32| x.pow(2);

fn main() {
    println!("Hello, world!");
}
