/// Tipe data Result
/// Ok untuk operasi sukses dan data
/// Err untuk operasi error dan keterangan
/// 
/// Result::Ok<T>
/// Result::Err<E>
/// 
/// dimana T dan E adalah generic
/// 
/// 
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InfinityNumber,
    OtherError
}

fn main() {
    let result1 = divider(10.0, 5.0);
    println!("result: {:?}", result1);

    let result2: Result<f64, MathError> = divider(10.0, 0.0);
    println!("result: {:?}", result2);

    // pattern matching untuk result
    let result = divider(10.0, 5.0);
    match result {
        Err(m) => println!("ERROR! {:?}", m),
        Ok(r)  => println!("result: {r:.2}"),
    }

    // dengan if condition
    let result = divider(10.0, 0.0);
    if result.is_err() {
        let err = result.as_ref().err();
        let message = err.unwrap();
        println!("error: {:?}", message);
        // error: DivisionByZero
    }

    if result.is_ok() {
        let data = result.as_ref().ok();
        let number = data.unwrap();
        println!("result: {:?}", number);
        // result: 2
    }

    let number = result.unwrap_or(0.0);
    println!("result: {}", number);
}

fn divider(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero);
    }

    let result = a / b;
    return Ok(result);
}