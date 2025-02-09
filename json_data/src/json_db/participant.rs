use std::{collections::HashMap, fs, path, sync::{Arc, Mutex}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub name: String,
    pub chip_id: String,
}

impl std::fmt::Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct ParticipantDb {
    // Mutex untuk mengamankan akses ke data bersama (multi-thread)
    data: Mutex<Vec<Participant>>,
    indexs: Mutex<HashMap<String, u32>>, // Menyimpan index berdasarkan chip_id
    file_path: String
}

impl ParticipantDb {
    pub fn new() -> Arc<Self> {
        // Arc adalah smart pointer untuk berbagi kepemilikan data antar thread
        let db = Arc::new(Self {
            data: Mutex::new(Vec::new()),
            indexs: Mutex::new(HashMap::new()),
            file_path: String::from("./dbs/participants.json")
        });
        db.init_file().ok();
        db.load().ok();
        db
    }

    pub fn all(&self) -> Vec<Participant> {
        // Mengembalikan salinan dari data dalam vector
        self.data.lock().unwrap().to_vec()
    }

    pub fn add(&self, p: Participant) {
        // Menambahkan peserta baru ke data
        self.data.lock().unwrap().push(p);
        self.indexing(); // Memperbarui index setelah data ditambah
    }

    pub fn get_by_chip_id(&self, value: &str) -> Option<Participant> {
        // Mendapatkan peserta berdasarkan chip_id
        let key = format!("chip_id_{}", value);
        let index = self.indexs.lock().unwrap().get(&key).cloned();
        
        // Jika ditemukan, ambil peserta berdasarkan index
        match index {
            Some(idx) => self.data.lock().unwrap().get(idx as usize).cloned(),
            None => None,
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file_path = self.file_path.as_str();
        let data = self.data.lock().unwrap();
        let json = serde_json::to_string_pretty(&*data)?; // Konversi ke JSON
        
        fs::write(file_path, json) // Tulis ke file
    }

    pub fn load(&self) -> Result<(), std::io::Error>  {
        println!("{}", "Load data");
        let file_path = self.file_path.as_str();
        let json = fs::read_to_string(file_path)?; // Baca file JSON
        println!("{}", json);
        let participants: Vec<Participant> = serde_json::from_str(&json)?; // Parse JSON
        
        println!("{}", "Load data done");

        // Masukkan data ke dalam Mutex
        let mut data = self.data.lock().unwrap();
        *data = participants;

        // tutup dulu lock pada data
        // jika tidak akan menyebabkan deadlock
        drop(data);
    
        // Lakukan indexing ulang
        self.indexing();
    
        Ok(())
    }

    fn init_file(&self) -> Result<(), std::io::Error> {
        let file_path = self.file_path.as_str();
        if !path::Path::new(file_path).exists() { 
            if let Some(parent) = std::path::Path::new(file_path).parent() {
                println!("{}", "buat file baru");
                fs::create_dir_all(parent)?; // Buat folder jika belum ada
                fs::write(file_path, "[]").ok();
            }
        }

        Ok(())
    }

    fn indexing(&self) {
        // Menyusun ulang index berdasarkan chip_id
        println!("{}", "Indexing data");

        let mut index = 0;
        let data = self.data.lock().unwrap();
        let mut indexs = self.indexs.lock().unwrap();
        indexs.clear();

        for p in data.iter() {
            indexs.insert(format!("chip_id_{}", p.chip_id), index);
            index += 1;
        }
        println!("{}", "Indexing done!");
    }
}

