// :dep reqwest = { version = "0.12", features = ["blocking"] }
// :dep zip = "2"

use std::io::{Read, Write};

fn main() {
    let years = 2011..2024 + 1;

    // Function to download ZIP file from URL and return a Reader
    fn download_zip(url: &str) -> std::io::Cursor<Vec<u8>> {
        let mut zip_buf: Vec<u8> = Vec::new();

        reqwest::blocking::get(url)
            .unwrap()
            .read_to_end(&mut zip_buf)
            .unwrap();
        std::io::Cursor::new(zip_buf)
    }

    // Function to extract a single .csv file from a ZIP archive and write it to ./data/lfs_csv
    fn write_csv(zip_file: &mut std::io::Cursor<Vec<u8>>, csv_name: &str) {
        let mut csv_buf: Vec<u8> = Vec::new();

        // Extract csv from buffer
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();
        let _ = archive
            .by_name(csv_name)
            .unwrap()
            .read_to_end(&mut csv_buf)
            .unwrap();

        // Write CSV file
        let mut file = std::fs::File::create(format!("./data/lfs_csv/{csv_name}")).unwrap();
        file.write_all(&csv_buf).unwrap();
    }

    // Create directory
    let _ = std::fs::remove_dir_all("./data");
    std::fs::create_dir("./data").unwrap();
    std::fs::create_dir("./data/lfs_csv").unwrap();
    std::fs::create_dir("./data/lfs_parquet").unwrap();
    std::fs::create_dir("./data/lfs_large").unwrap();
    std::fs::create_dir("./data/temp_data").unwrap();
    std::fs::create_dir("./data/minio").unwrap();

    // For the full-year files (prior to current year)
    for y in years {
        let url = format!("https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/{y}-CSV.zip");

        let mut zip = download_zip(&url);

        for m in 1..(12 + 1) {
            let mm = format!("{:02}", m);
            let yy = format!("{:02}", y % 2000);

            write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
        }
    }
}
