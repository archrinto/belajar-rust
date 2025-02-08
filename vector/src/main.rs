fn main() {
    // vector seperti array tapi dinamis
    // tipe data wajib sama
    // punya len dan capacity, jika isi lebih dari capacity
    // maka capacity akan ditambah
    let mut data_one = vec!["batman", "superman", "lobo"];

    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // pop, menghapus element terakhir

    data_one.pop();

    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());    

    // remove, menghapus element di index ke-i
    data_one.remove(0);

    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());   

    // push, menambahkan data
    data_one.push("constantine");
    data_one.push("trigon");
    data_one.push("darkseid");

    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(),  data_one.capacity());

    // is_empty
    let is_vector_empty = data_one.is_empty();
    println!("result: {:?}", is_vector_empty);  

    // clear, mengosongkan vector
    data_one.clear();
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(),  data_one.capacity());


    // append, menambahkan vector lain

    let mut result_one = vec![3, 1, 2];

    let mut data_two = vec![7, 6, 8];
    // harus bentuk mut reference
    result_one.append(&mut data_two);

    println!("data: {:?}", result_one);
    println!("length: {}, capacity: {}", result_one.len(),  result_one.capacity());

    // sort, mengurutkan vector
    println!("data: {:?}", result_one);
    result_one.sort();
    println!("data: {:?}", result_one);

    // deklarasi vector
    // let mut vector_4 = vec![1, 2, 3];
    // let mut vector_5: Vec<i64> = vec![1, 2, 3];

    // jika deklarasi dengan data kosong
    // wajib mendeklarasikan tipe datanya
    // let mut vector_7: Vec<&str> = vec![];
    // let mut vector_8: Vec<&str> = Vec::new();

    // iterasi pada vector
    let vec_eight = vec![1, 2, 3];
    for e in vec_eight {
        print!("{e} ");
    }

    let vec_nine = vec![1, 2, 3];
    for i in 0..vec_nine.len() {
        print!("{} ", vec_nine[i]);
    }

    // ownership pada vector
    let vec_ten = vec![1, 2, 3];
    for e in &vec_ten {
        print!("{e} ");
    }
    // pada loop diatas, data di vec_ten berpindah
    // ownership ke e
    // ini disebut move semantic

    // membuat kode ini menjadi error
    for i in 0..vec_ten.len() {
        print!("{} ", vec_ten[i]);
    }

    println!();

    // caranya untuk fixingya adalah menggunakan teknik borrowing 
    // menggunakan operator reference


    // slice, mirip seperti array 
    let vec_population = vec![1, 2, 3];
    let vec_sample = &vec_population[0..1];
    println!("{:?}", vec_sample);


    // VecDeque
    // memungkinakan vector memiliki fungsi untuk
    // menambah atau mengurangi element dari depan dan belakang
    use std::collections::VecDeque;
    
    let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);
    
    // pop_front, mengambil element paling depan
    vec_10.pop_front();

    // push_front, menambahkan element paling depan
    vec_10.push_front("z");
    println!("data: {:?}", vec_10);

    // pop_back, mengambil element paling belakang
    vec_10.pop_back();
    // push_back, menambahkan element paling belakang
    vec_10.push_back("h");
    println!("data: {:?}", vec_10);

}
