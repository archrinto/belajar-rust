mod json_db;

use json_db::participant::{Participant, ParticipantDb};

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
}