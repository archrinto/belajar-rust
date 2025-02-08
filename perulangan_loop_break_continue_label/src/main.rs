// use std::thread::sleep;
// use std::time::Duration;

fn main() {
    let mut i = 0;
    let max = 5;
    loop {
        // println!("nilai: {i}");
        let mut j = max;
        let max_inner = i;
        
        loop {
            print!("* ");
            j -= 1;
            if j < max_inner {
                break;
            }
        }
        
        println!();
        
        i += 1;
        if i > max {
            break;
        }

        if i % 2 == 0 {
            continue;
        }

        println!("-------");

        // sleep(Duration::from_secs(1));
    }

    // loop dengan label
    // bisa digunakan untuk melakukan break terhadap 
    // loop diluar block kode
    let mut i2 = 0;
    let max2 = 9;
    'mainLoop: loop {
        i2 += 1;
        let mut j = 0;

        loop {
            if i2 > max2 {
                break 'mainLoop;
            }

            j += 1;
            if j > i2 {
                break;
            }

            print!("{i2} ");
        }
        println!();
    }

    // pengembalian nilai dari loop

    let mut counter = 0;
    let result = loop {
        counter += 1;

        // return disini akan menghasilkan error
        // if counter == 5 {
        //     return counter;
        // }

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}
