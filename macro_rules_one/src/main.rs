/// Macro Rules
/// macro mirip seperti function, namun pemanggilannya menggunakan !
/// 
/// contoh deklarasinya seperti ini
macro_rules! say_hello {
    () => {
        println!("hello");
    }
}

/// macro dengan argument
/// menggunakan prefix $ dan dengan tipe designator
/// designator:
/// - ident: untuk variable atau nama fungsi
/// - expr: untuk sebuah expression
/// - stmt: statement
/// dll.
/// 

macro_rules! create_function {
    // macro untuk membuat fungsi
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    }
}

macro_rules! print_result {
    // macro untuk menerima expresi dan mencetak hasilnya
    ($expres:expr) => {
        println!("{:?} = {:?}", stringify!($expres), $expres);
    }
}

/// sepertinya deklarasi dari sebuah macro ini block codenya mirip
/// dengan match, sesuai dengan expression yang ada
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

/// macro juga dapat menggunakan dekalrasi jumlah parameter dinamis
/// + untuk paling tidak satu
/// * untuk mungkin ada atau lebih dari satu

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr,$($y:expr),+) => (std::cmp::min($x, find_min!($($y),+)))
}

create_function!(foo);

fn main() {
    say_hello!();
    foo();
    print_result!(1 + 2 + 3);
    print_result!({
        let x = 10;
        x * x + 2
    });
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}