pub trait Shape {
    type Area; // <-- definisi type di trait

    fn area(&self) -> Self::Area;
}