use std::io;

fn main() {
    println!("Selamat datang di aplikasi my collection");

    println!("Pilih opsi yang ingin Anda lakukan:");
    println!("1. Menu");
    

    let mut pilihan = String::new();
    io::stdin().read_line(&mut pilihan).unwrap();

    let pilihan: u8 = pilihan.trim().parse().unwrap();

    match pilihan {
        1 => login(),
        
        _ => println!("Pilihan yang Anda masukkan tidak valid"),
    }
}

fn login() {
    // Kode untukfn main() {
    let mut data_base: HashMap<String, String> = HashMap::new();
    let mut selection: String = String::new();

    loop {
        println!("my collection");
        println!("1. Tambahkan");
        println!("2. Lihat ");
        println!("3. Ubah ");
        println!("4. Hapus ");
        println!("5. Keluar");

        selection = String::new();
        io::stdin().read_line(&mut selection)
            .expect("Gagal membaca baris");

        match &selection[..] {
            "1\n" => add_entry(&mut data_base),
            "2\n" => view_entry(&data_base),
            "3\n" => edit_entry(&mut data_base),
            "4\n" => delete_entry(&mut data_base),
            "5\n" => break,
            _ => println!("Silakan masukkan pilihan yang valid"),
        }
    }
}

fn add_entry(data_base: &mut HashMap<String, String>) {
    let mut key: String = String::new();
    let mut value: String = String::new();

    println!("Masukkan barang: ");
    io::stdin().read_line(&mut key)
        .expect("Gagal membaca baris");

    println!("Masukkan jumlah: ");
    io::stdin().read_line(&mut value)
        .expect("Gagal membaca baris");

    data_base.insert(key.trim().to_string(), value.trim().to_string());
}

fn view_entry(data_base: &HashMap<String, String>) {
    let mut key: String = String::new();

    println!("Masukkan barang: ");
    io::stdin().read_line(&mut key)
        .expect("Gagal membaca baris");

    match data_base.get(&key.trim().to_string()) {
        Some(value) => println!("jumlah: {}", value),
        None => println!("Tidak ada entri yang ditemukan untuk kunci ini"),
    }
}

fn edit_entry(data_base: &mut HashMap<String, String>) {
    let mut key: String = String::new();
    let mut value: String = String::new();

    println!("Masukkan barang: ");
    io::stdin().read_line(&mut key)
        .expect("Gagal membaca baris");

    match data_base.get(&key.trim().to_string()) {
        Some(_) => {
            println!("Masukkan jumlah baru: ");
            io::stdin().read_line(&mut value)
                .expect("Gagal membaca baris");

            data_base.insert(key.trim().to_string(), value.trim().to_string());
        }
        None => println!("Tidak ada entri yang ditemukan untuk kunci ini"),
    }
}

fn delete_entry(data_base: &mut HashMap<String, String>) {
    let mut key: String = String::new();

    println!("Masukkan barang: ");
    io::stdin().read_line(&mut key)
        .expect("Gagal membaca baris");

    data_base.remove(&key.trim().to_string());
} proses login
}

