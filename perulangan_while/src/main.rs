use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let max_inner = i;

        while j <= max_inner {
            print!("*");
            j += 1;
        }
        i += 1;
        println!()
    }

    i = 0;

    // menambahkan delay di while loop
    while i < max {
        println!("nilai: {i}");
        i += 1;

        sleep(Duration::from_secs(1));
    }
}
