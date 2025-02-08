// Pengelolaan memory oleh program

// dalam bahasa pemograman terdapat beberapa management memory:
// 1. Garbage Collection
//      management memory otomatis. secara berkala garbage collector
//      akan melakukan pengecekan secara berkala terhadap penggunaan
//      memory oleh program (memonitor), kemudian melakukan dealokasi
//      proses dealokasi terjadi secara async.
//      Java, C#, Go

// 2. Automactic reference counting (ARC)
//      diterapkan pada bahasa Obejective-C dan Swift
// 
// 3. Manual memory
//      programer memiliki tanggung jawab untuk management memory
//      kapan waktunya dan di mana lokasinya (heap autu stack)
//      dipakai oleh C dan C++
//
// 4. Ownership rule
//      digunakan rust
//
// Memory Address, alamat spesifik di memory yang digunakan 
// sofware atau hardware untuk menyimpan data.
// alokasi memory tidak pengaruh pada value
// melaikan tipe datanya. 
// contoh i32, maka alokasi di memory adalah 32 bit atau 3 bytes
// berapapun valuenya (1, 10, 25, 100 etc)

// stack memory
// LIFO (terakhir masuk pertama kali keluar)
// untuk data yang sudah diketahui size / ukurannya, batasnya
// alokasi bersifat local terhadap pemanggilan fungsi
// kecepatan pengaksesan tinggi
// data primitif disimpan di stack (i32, bool dll.)

// heap memory
// alokasi memory selain stack
// untuk data dengan alokasi data dinamis, tidak diketahui sizenya, sizenya bisa berubah
// tidak punya pattern tertentu
// lebih lambat dari stack
// data non primitif


fn main() {
    println!("Hello, world!");
}
