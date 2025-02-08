pub struct Circle {
    pub radius: f64,
}

impl crate::shape::Shape for Circle {
    type Area = f64; // <-- deklarasi tipe data konkret saat implementasi

    fn area(&self) -> Self::Area {
        std::f64::consts::PI * self.radius * self.radius
    }
}