fn main() {
    /// Static
    /// mirip dengan konstanta
    /// dibuat menggunakan 
    /// - static pada pendefinisian konstanta
    /// - 'static pada tipe data string literal
    /// 
    /// bisa digunakan untuk semua tipe data primitif
    /// bisa juga digunakan untuk  static constant function
    
    static PI: f64 = 3.14;

    /// static bisa juga mutable tapi jadi unsafe
    /// 
    /// Lifetime static tidak akan didealokasi kecuali eksekusi
    /// program selesai.
    /// 
    /// Karena alasan itu, lebih baik data dengan lifetime static
    /// dideklarasikan secara global
    /// 
    /// dengan alasan itu, ketika static didalam block, walaupun
    /// blocknya selesai variable tersebut tidak didealokasi
    /// 
    const VERSION: &'static str = "v1.2.3";
    // sama dengan
    // const VERSION: &str = "v1.2.3";
    /// &str tidak memiliki owner
    /// dengan adanya lifetime static data borrow tidak akan pernah
    /// didalokasi

    println!("Hello, world!");
}
