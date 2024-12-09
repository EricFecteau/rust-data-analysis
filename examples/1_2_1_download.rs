use reqwest::blocking::get;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

fn main() {
    let current_year = 2024;
    let current_month = 9; // Latest month the LFS is available

    // Function to download ZIP file from URL and return a Reader
    fn download_zip(url: &str) -> Cursor<Vec<u8>> {
        let mut zip_buf: Vec<u8> = Vec::new();

        get(url).unwrap().read_to_end(&mut zip_buf).unwrap();
        std::io::Cursor::new(zip_buf)
    }

    // Function to extract a single .csv file from a ZIP archive and write it to ./data/lfs_csv
    fn write_csv(zip_file: &mut Cursor<Vec<u8>>, csv_name: &str) {
        let mut csv_buf: Vec<u8> = Vec::new();

        // Extract csv from buffer
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();
        let _ = archive
            .by_name(csv_name)
            .unwrap()
            .read_to_end(&mut csv_buf)
            .unwrap();

        // Write CSV file
        let mut file = File::create(format!("./data/lfs_csv/{csv_name}")).unwrap();
        file.write_all(&csv_buf).unwrap();
    }

    // Create directory
    let _ = fs::remove_dir_all("./data");
    fs::create_dir("./data").unwrap();
    fs::create_dir("./data/lfs_csv").unwrap();
    fs::create_dir("./data/lfs_parquet").unwrap();
    fs::create_dir("./data/lfs_large").unwrap();

    // For the full-year files (prior to current year)
    for y in 2006..current_year {
        let url = format!("https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/{y}-CSV.zip");

        let mut zip = download_zip(&url);

        for m in 1..(12 + 1) {
            let mm = format!("{:02}", m);
            let yy = format!("{:02}", y % 2000);

            write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
        }
    }

    // For the monthly file in the current year
    for m in 1..(current_month + 1) {
        let mm = format!("{:02}", m);
        let yy = format!("{:02}", current_year % 2000);

        let url = format!(
            "https://www150.statcan.gc.ca/n1/en/pub/71m0001x/2021001/{current_year}-{mm}-CSV.zip"
        );

        let mut zip = download_zip(&url);
        write_csv(&mut zip, &format!("pub{mm}{yy}.csv"));
    }
}
