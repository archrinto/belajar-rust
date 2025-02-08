use rand::Rng;

fn generate_random_number() -> i32 {
    return rand::thread_rng().gen_range(0..100);
}

fn main() {
    for u in 0..5 {
        println!("random number {}: {}", u, generate_random_number());
    }
}
