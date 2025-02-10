mod json_db;

use json_db::{categories::{CategoryDb, Category}, participant::{Participant, ParticipantDb}, Database};

fn main() {
    println!("main is running..");
    let participant_db = ParticipantDb::new();

    // println!("create new participant");
    // let p1 = Participant{
    //     name: String::from("Budi"),
    //     chip_id: String::from("00001")
    // };
    // println!("{}", p1);

    // println!("store participant");
    // participant_db.add(p1);
    // participant_db.save().ok();

    let p = participant_db.get_by_chip_id("00001");
    println!("{:?}", p);

    let mut category_db = CategoryDb::new();
    category_db.add(&Category {
        id: 1,
        name: String::from("Category 1")
    });
    println!("category: {:?}", category_db.get_by_index("id", "1"));
    category_db.remove_by_index("id", "1");
    println!("category: {:?}", category_db.get_by_index("id", "1"));
}