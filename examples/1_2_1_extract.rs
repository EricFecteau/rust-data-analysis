// === imports
use std::{
    fs::File,
    io::{Read, Write},
};

// === main
fn main() {
    // === program

    // Create directories used throughout the book
    let _ = std::fs::remove_dir_all("./data");
    std::fs::create_dir("./data").unwrap();
    std::fs::create_dir("./data/raw").unwrap();
    std::fs::create_dir("./data/codeset").unwrap();
    std::fs::create_dir("./data/csv").unwrap();
    std::fs::create_dir("./data/parquet").unwrap();
    std::fs::create_dir("./data/large").unwrap();
    std::fs::create_dir("./data/temp_data").unwrap();
    std::fs::create_dir("./data/minio").unwrap();
    std::fs::create_dir("./data/output").unwrap();

    // Open ZIP file
    let zip_file = File::open("./zip/data.zip").unwrap();

    // Initiate CSV buffer
    let mut csv_buf: Vec<u8> = Vec::new();

    // Find census.csv in Zip
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let _ = archive
        .by_name("census.csv")
        .unwrap()
        .read_to_end(&mut csv_buf)
        .unwrap();

    // Write `census.csv` file
    let mut file = std::fs::File::create("./data/raw/census.csv").unwrap();
    file.write_all(&csv_buf).unwrap();

    // Clear buffer
    csv_buf.clear();

    // Find codeset.csv in Zip
    let _ = archive
        .by_name("codeset.csv")
        .unwrap()
        .read_to_end(&mut csv_buf)
        .unwrap();

    // Write `codeset.csv` file
    let mut file = std::fs::File::create("./data/codeset/codeset.csv").unwrap();
    file.write_all(&csv_buf).unwrap();

    // === end
}
