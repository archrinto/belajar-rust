mod models;

struct User {
    name: String,
    sign_in_count: u32,
    affliation: Vec<String>,
    active: bool,
}

fn main() {
    // object user one
    let user_one = User {
        name: String::from("Ridho awaludin"),
        sign_in_count: 12,
        affliation: vec![
            String::from("Dru Ewake"),
            String::from("Blueye Maggo"),
            String::from("black bone"),
        ],
        active: false,
    };

    println!("name: {}", user_one.name);
    println!("sign-in count: {}", user_one.sign_in_count);
    println!("affliation: {:?}", user_one.affliation);
    println!("is active? {}", user_one.active);

    // variable struct dengan nilai berasal dari variable lain
    struct Car {
        brand: String,
        model: String,
    }

    let mut car_three: Car;
    car_three = Car{
        brand: String::from("Audi"),
        model: String::from("Le Mans Quattro"),
    };
    println!("{} {}", car_three.brand, car_three.model);

    let mut car_four: Car;
    car_four = Car{
        brand: String::from("Audi Brand"),
        ..car_three
    };
    println!("{} {}", car_four.brand, car_four.model);

    // shorthand
    let model = String::from("Corvette C1");
    let car_five = Car{
        brand: String::from("Chevrolet"),
        model, // <- jika nama prop dan variable namanya sama
    };

    struct Point {
        x: f32,
        y: f32,
    }
    
    let point_one = Point { x: 3.14, y: 8.0 };
    
    // deklarasi destructuring assignment
    let Point { x: x_one, y: y_one } = point_one;
    let Point { x, y } = point_one;
    println!("x_one: {}", x_one);
    println!("y_one: {}", y_one);

    // debunging value struct
    // dilakukan dengan menggunakan derive (debug)
    struct Gamebot {
        name: String
    }

    #[derive(Debug)]
    struct GamingConsole {
        name: String
    }

    let gamebot_one = Gamebot{
        name: String::from("Gamebot 5"),
    };

    let console_one = GamingConsole{
        name: String::from("PlayStation 5"),
    };

    // ini kan error
    // println!("data_struct_one: {:#?}", gamebot_one);

    // ini tidak
    println!("data_struct_one: {:#?}", console_one);

    // tuple struct
    struct Color(i32, i32, i32);

    let red = Color(255, 0, 0);

    println!("{:?} {:?} {:?}", red.0, red.1, red.2);

    let motor = models::motor::Motor {
        brand: String::from("Yamaha"),
    };
}
