/// Option
/// Option::Some<T> atau Some<T> untuk menandai data memiliki value
/// Option::None atau None, untuk mendandai data tidak ada nilainya

fn divider(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    let result = a / b;
    return Some(result);
}

fn main() {
    let result1 = divider(10, 5);
    println!("result: {:?}", result1);
    match result1 {
        None => println!("cannot divide by 0"),
        Some(2) => println!("the result is two"),
        Some(x) => println!("result: {x}"),
    }

    let result2: Option<i32> = divider(10, 0);
    println!("result: {:?}", result2);
    match result2 {
        None => println!("cannot divide by 0"),
        Some(x) => {
            println!("result: {}", x)
        }
    }

    if result1 != None {
        let number = result1.unwrap();
        println!("result unwrap: {}", number)
    }

    if result1.is_some() {
        let number = result1.unwrap();
        println!("result: {}", number);
    }
    
    if !result1.is_none() {
        let number = result1.unwrap();
        println!("result: {}", number);
    }

    /// pilihan lain menggunakan 
    /// unwrap_or_default()
    /// jika i32 maka defaultnya adalah 0
    /// 
    /// atau
    /// 
    /// unwrap_or(default_value)
    /// 
    /// atau unwrap_or_else(|| 0)
}
